use crate::errors::{Successful, WebError, WebResult};
use crate::forms::documents::document::Document;
use crate::forms::documents::vector::DocumentVectors;
use crate::forms::documents::forms::DocumentType;
use crate::forms::documents::preview::DocumentPreview;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::documents::helper as d_helper;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::DocumentService;
use crate::services::searcher::elastic::documents::update::UpdateTrait;

use elasticsearch::DeleteParts;
use elasticsearch::http::Method;
use elasticsearch::params::Refresh;
use serde::Deserialize;
use serde_json::Value;

#[async_trait::async_trait]
impl DocumentService for ElasticContext {
    async fn create_document(&self, folder_id: &str, doc_form: &Document, doc_type: &DocumentType) -> WebResult<Successful> {
        let elastic = self.get_cxt().read().await;
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
        let elastic = self.get_cxt().read().await;
        let s_doc_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response =
            helper::send_elrequest(&elastic, Method::Get, None, s_doc_path.as_str()).await?;

        d_helper::extract_document(response).await
    }
    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Successful> {
        let elastic = self.get_cxt().read().await;
        let response = elastic
            .delete(DeleteParts::IndexId(folder_id, doc_id))
            .refresh(Refresh::True)
            .timeout("1m")
            .send()
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn update_document(&self, folder_id: &str, value: &Value, doc_type: &DocumentType) -> WebResult<Successful> {
        match doc_type {
            DocumentType::Preview => {
                let doc = DocumentPreview::deserialize(value)?;
                DocumentPreview::update(self, folder_id, &doc).await
            }
            _ => {
                let doc = Document::deserialize(value)?;
                Document::update(self, folder_id, &doc).await
            }
        }
    }
}
