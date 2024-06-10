use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::forms::documents::DocumentsTrait;
use crate::forms::documents::document::Document;
use crate::forms::documents::embeddings::DocumentVectors;
use crate::forms::documents::forms::{DocumentType, MoveDocsForm};
use crate::services::notifier::notifier;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::documents::helper as d_helper;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::DocumentService;

use elasticsearch::http::Method;
use serde::Deserialize;
use serde_json::Value;
use crate::forms::documents::preview::DocumentPreview;
use crate::services::searcher::elastic::documents::update::UpdateTrait;

#[async_trait::async_trait]
impl DocumentService for ElasticContext {
    async fn create_document(&self, doc_form: &Document, doc_type: &DocumentType) -> WebResult<Successful> {
        let doc_id = doc_form.get_doc_id();
        let elastic = self.get_cxt().read().await;
        let is_exists = d_helper::check_duplication(&elastic, doc_form).await?;
        if is_exists {
            let msg = format!("Passed document: {} already exists", doc_id);
            let entity = WebErrorEntity::new(msg);
            return Err(WebError::CreateDocument(entity));
        }

        match doc_type {
            DocumentType::Vectors => {
                let doc_vecs = DocumentVectors::from(doc_form);
                d_helper::store_object::<DocumentVectors>(&elastic, &doc_vecs).await
            }
            _ => d_helper::store_object::<Document>(&elastic, doc_form).await
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
        let s_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response =
            helper::send_elrequest(&elastic, Method::Delete, None, s_path.as_str()).await?;
        helper::parse_elastic_response(response).await
    }
    async fn move_documents(&self, folder_id: &str, move_form: &MoveDocsForm) -> WebResult<Successful> {
        let cxt_opts = self.get_options();
        let move_result = notifier::move_docs_to_folder(cxt_opts.as_ref(), folder_id, move_form)
            .await
            .map_err(WebError::from)?;

        if !move_result.is_success() {
            let msg = format!("Failed while moving documents from: {}", folder_id);
            log::error!("{}", msg.as_str());
            let entity = WebErrorEntity::new(msg);
            return Err(WebError::MoveDocuments(entity));
        }

        let mut collected_errs = Vec::default();
        for doc_id in move_form.get_doc_ids() {
            let result = d_helper::move_document(self, doc_id, folder_id, move_form).await;
            if result.is_err() {
                let err = result.err().unwrap();
                let msg = format!("Failed to move document {}: {}", doc_id, err);
                collected_errs.push(msg);
            }
        }

        if !collected_errs.is_empty() {
            let collected_docs = collected_errs.join(", ");
            let msg = format!("Failed while move document: {}", collected_docs);
            let entity = WebErrorEntity::new(msg);
            return Err(WebError::MoveDocuments(entity));
        }

        Ok(Successful::success("Ok"))
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
