use crate::errors::{JsonResponse, SuccessfulResponse, WebError};
use crate::forms::document::{Document, DocumentPreview, MoveDocumetsForm};
use crate::services::elastic::{context, helper};
use crate::services::notifier::notifier;
use crate::services::searcher::DocumentsService;

use actix_web::web;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use serde_json::Value;

#[async_trait::async_trait]
impl DocumentsService for context::ElasticContext {
    async fn get_document(&self, folder_id: &str, doc_id: &str) -> JsonResponse<Document> {
        let elastic = self.get_cxt().read().await;
        let s_doc_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response = elastic
            .send(
                Method::Get,
                s_doc_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        let document = helper::extract_document(response).await?;
        Ok(web::Json(document))
    }
    async fn create_document(&self, doc_form: &Document) -> Result<SuccessfulResponse, WebError> {
        let doc_id = doc_form.get_doc_id();
        let folder_id = doc_form.get_folder_id();
        let elastic = self.get_cxt().read().await;
        let is_exists = helper::check_duplication(&elastic, folder_id, doc_id).await?;
        if is_exists {
            let msg = format!("Passed document: {} already exists", doc_id);
            return Err(WebError::CreateDocument(msg));
        }

        helper::store_document(&elastic, doc_form, folder_id).await
    }
    async fn create_document_preview(
        &self,
        folder_id: &str,
        doc_form: &DocumentPreview,
    ) -> Result<SuccessfulResponse, WebError> {
        // TODO: Impled for Document and DocumentPreview into create_document()
        let elastic = self.get_cxt().read().await;
        helper::store_doc_preview(&elastic, doc_form, folder_id).await
    }
    async fn update_document(&self, doc_form: &Document) -> Result<SuccessfulResponse, WebError> {
        // TODO: Impled for Document and DocumentPreview
        let elastic = self.get_cxt().read().await;
        let document_json = serde_json::to_value(doc_form).map_err(WebError::from)?;

        let doc_id = doc_form.get_doc_id();
        let folder_id = doc_form.get_folder_id();
        let s_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response = elastic
            .send(
                Method::Put,
                s_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(document_json.to_string().as_bytes()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn delete_document(
        &self,
        folder_id: &str,
        doc_id: &str,
    ) -> Result<SuccessfulResponse, WebError> {
        let elastic = self.get_cxt().read().await;
        let s_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response = elastic
            .send(
                Method::Delete,
                s_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn move_documents(
        &self,
        move_form: &MoveDocumetsForm,
    ) -> Result<SuccessfulResponse, WebError> {
        let opts = self.get_options();
        let move_result = notifier::move_docs_to_folder(opts.as_ref(), move_form)
            .await
            .map_err(WebError::from)?;

        let src_folder_id = move_form.get_src_folder_id();
        if !move_result.is_success() {
            let msg = format!("Failed while moving documents from: {}", src_folder_id);
            log::error!("{}", msg.as_str());
            return Err(WebError::MoveDocuments(msg));
        }

        let mut collected_errors = Vec::default();
        for document_id in move_form.get_document_ids() {
            let result = self.get_document(src_folder_id, document_id).await;
            if result.is_err() {
                let err = result.err().unwrap();
                log::error!("Failed while move document: {}", err.to_string());
                collected_errors.push(document_id.to_owned());
                continue;
            }

            let document = result.unwrap().0;

            let result = self.delete_document(src_folder_id, document_id).await;
            if result.is_err() {
                let err = result.err().unwrap();
                log::error!("Failed while move document: {}", err.to_string());
                collected_errors.push(document_id.to_owned());
                continue;
            }

            let result = self.create_document(&document).await;
            if result.is_err() {
                let err = result.err().unwrap();
                log::error!("Failed while move document: {}", err.to_string());
                collected_errors.push(document_id.to_owned());
            }
        }

        if collected_errors.len() > 0 {
            let collected_docs = collected_errors.join(", ");
            let msg = format!("Failed while move document: {}", collected_docs);
            return Err(WebError::MoveDocuments(msg));
        }

        Ok(SuccessfulResponse::success("Ok"))
    }
}
