pub mod helper;
pub mod schema;
mod store;
mod update;

use crate::elastic::ElasticClient;
use crate::errors::{Successful, WebError, WebResult};
use crate::searcher::elastic::helper as s_helper;
use crate::storage::elastic::helper as f_helper;
use crate::storage::elastic::helper as d_helper;
use crate::storage::elastic::update::UpdateTrait;
use crate::storage::forms::CreateFolderForm;
use crate::storage::forms::DocumentType;
use crate::storage::models::Document;
use crate::storage::models::DocumentPreview;
use crate::storage::models::DocumentVectors;
use crate::storage::models::Folder;
use crate::storage::models::InfoFolder;
use crate::storage::DocumentService;
use crate::storage::FolderService;

use elasticsearch::http::Method;
use elasticsearch::params::Refresh;
use elasticsearch::DeleteParts;
use serde::Deserialize;
use serde_json::Value;

const CAT_INDICES_URL: &str = "/_cat/indices?format=json";

#[async_trait::async_trait]
impl FolderService for ElasticClient {
    async fn get_all_folders(&self, show_all: bool) -> WebResult<Vec<Folder>> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response =
            s_helper::send_elrequest(&elastic, Method::Get, None, CAT_INDICES_URL).await?;

        let folders = response.json::<Vec<Folder>>().await?;
        f_helper::filter_folders(&elastic, folders, show_all).await
    }

    async fn get_folder(&self, folder_id: &str) -> WebResult<Folder> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let target_url = format!("/{folder_id}/_stats");
        let response =
            s_helper::send_elrequest(&elastic, Method::Get, None, target_url.as_str()).await?;

        let json_value = response.json::<Value>().await?;
        let mut folder = f_helper::extract_folder_stats(&json_value)?;
        let _ = f_helper::load_info_doc(&elastic, &mut folder).await;
        Ok(folder)
    }

    async fn create_folder(&self, folder_form: &CreateFolderForm) -> WebResult<Successful> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response = f_helper::create_index(&elastic, folder_form).await?;
        let result = s_helper::parse_elastic_response(response).await?;
        if result.is_success() {
            let info_folder = InfoFolder::from(folder_form);
            let res = d_helper::store_object(&elastic, "info-folder", &info_folder).await?;
            tracing::warn!("{} - {}", res.code, res.message);
        }

        Ok(result)
    }

    async fn delete_folder(&self, folder_id: &str) -> WebResult<Successful> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response = s_helper::send_elrequest(&elastic, Method::Delete, None, folder_id).await?;
        s_helper::parse_elastic_response(response).await
    }
}

#[async_trait::async_trait]
impl DocumentService for ElasticClient {
    async fn create_document(
        &self,
        folder_id: &str,
        doc_form: &Document,
        doc_type: &DocumentType,
    ) -> WebResult<Successful> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        match doc_type {
            DocumentType::Vectors => {
                let docs_vec = DocumentVectors::from(doc_form);
                d_helper::store_objects::<DocumentVectors>(&elastic, folder_id, &docs_vec).await
            }
            _ => {
                let mut doc_cln = doc_form.clone();
                doc_cln.exclude_tokens();
                d_helper::store_object::<Document>(&elastic, folder_id, &doc_cln).await
            }
        }
    }

    async fn get_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Document> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let s_doc_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response =
            s_helper::send_elrequest(&elastic, Method::Get, None, s_doc_path.as_str()).await?;

        d_helper::extract_document(response).await
    }

    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Successful> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response = elastic
            .delete(DeleteParts::IndexId(folder_id, doc_id))
            .refresh(Refresh::True)
            .timeout("1m")
            .send()
            .await
            .map_err(WebError::from)?;

        s_helper::parse_elastic_response(response).await
    }

    async fn update_document(
        &self,
        folder_id: &str,
        value: &Value,
        doc_type: &DocumentType,
    ) -> WebResult<Successful> {
        let client = self.es_client();
        match doc_type {
            DocumentType::Preview => {
                let doc = DocumentPreview::deserialize(value)?;
                DocumentPreview::update(client, folder_id, &doc).await
            }
            _ => {
                let doc = Document::deserialize(value)?;
                Document::update(client, folder_id, &doc).await
            }
        }
    }
}
