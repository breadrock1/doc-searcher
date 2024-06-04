use crate::errors::{Successful, WebError, WebResult};
use crate::forms::documents::DocumentsTrait;
use crate::forms::documents::document::Document;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::documents::helper as d_helper;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::DocumentsService;

use elasticsearch::http::Method;
use serde_json::Value;

#[async_trait::async_trait]
impl DocumentsService for ElasticContext {
    async fn get_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Document> {
        let elastic = self.get_cxt().read().await;
        let s_doc_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response =
            helper::send_elrequest(&elastic, Method::Get, None, s_doc_path.as_str()).await?;

        d_helper::extract_document(response).await
    }
    async fn create_document(&self, doc_form: &Document) -> WebResult<Successful> {
        let doc_id = doc_form.get_doc_id();
        let elastic = self.get_cxt().read().await;
        let is_exists = d_helper::check_duplication(&elastic, doc_form).await?;
        if is_exists {
            let msg = format!("Passed document: {} already exists", doc_id);
            return Err(WebError::CreateDocument(msg));
        }

        d_helper::store_document(&elastic, doc_form).await
    }
    async fn update_document(&self, folder_id: &str, doc_id: &str, value: &Value) -> WebResult<Successful> {
        let elastic = self.get_cxt().read().await;
        let s_path = format!("/{}/_update/{}", folder_id, doc_id);
        let mut bytes: Vec<u8> = Vec::new();
        serde_json::to_writer(&mut bytes, value).unwrap();
        let response = helper::send_elrequest(
            &elastic,
            Method::Put,
            Some(bytes.as_slice()),
            s_path.as_str(),
        )
        .await?;
        helper::parse_elastic_response(response).await
    }
    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Successful> {
        let elastic = self.get_cxt().read().await;
        let s_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response =
            helper::send_elrequest(&elastic, Method::Delete, None, s_path.as_str()).await?;
        helper::parse_elastic_response(response).await
    }
}
