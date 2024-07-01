use doc_search::errors::{Successful, WebError, WebResult};
use doc_search::forms::documents::document::Document;
use doc_search::forms::documents::forms::{DocumentType, MoveDocsForm};
use doc_search::services::own_engine::context::OtherContext;
use doc_search::services::service;

#[async_trait::async_trait]
impl service::DocumentsService for OtherContext {
    async fn get_document(&self, _bucket_id: &str, doc_id: &str) -> WebResult<Document> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        match map.get(doc_id) {
            Some(document) => Ok(document.clone()),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while getting bucket: {}", msg.as_str());
                Err(WebError::GetDocument(msg))
            }
        }
    }

    async fn create_document(
        &self,
        doc_form: &Document,
        _doc_type: &DocumentType,
    ) -> WebResult<Successful> {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.documents.write().await;
        let doc_name = doc_form.get_doc_name();
        match map.insert(doc_name.to_string(), doc_form.clone()) {
            Some(_document) => Ok(Successful::success("Ok")),
            None => {
                let msg = format!("Created {}", doc_name);
                log::warn!("Failed while creating document: {}", msg.as_str());
                Ok(Successful::success(msg.as_str()))
            }
        }
    }

    async fn update_document(&self, doc_form: &Document) -> WebResult<Successful> {
        self.create_document(&doc_form, &DocumentType::Document).await
    }

    async fn delete_document(&self, _bucket_id: &str, doc_id: &str) -> WebResult<Successful> {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.documents.write().await;
        match map.remove(doc_id) {
            Some(_) => Ok(Successful::success("Ok")),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while deleting document: {}", msg.as_str());
                Err(WebError::DeleteDocument(msg))
            }
        }
    }
}

#[cfg(test)]
mod test_documents {
    use crate::forms::documents::document::{Document, DocumentBuilderError};
    use crate::forms::documents::forms::DocumentType;
    use crate::services::own_engine::context::OtherContext;
    use crate::services::service::DocumentsService;

    use actix_web::test;

    const FOLDER_ID: &str = "test-folder";
    const DOCUMENT_ID: &str = "test-document";

    fn create_default_document(document_name: &str) -> Result<Document, DocumentBuilderError> {
        Document::builder()
            .folder_id(FOLDER_ID.to_string())
            .folder_path("/test-folder".to_string())
            .content("Any document text".to_string())
            .embeddings(Vec::default())
            .document_name(document_name.to_string())
            .document_path(format!("/test-folder/{}", document_name))
            .document_size(1024)
            .document_type("document".to_string())
            .document_extension(".txt".to_string())
            .document_permissions(777)
            .document_id("md5 hash".to_string())
            .document_ssdeep("ssdeep hash".to_string())
            .document_created(None)
            .document_modified(None)
            .highlight(None)
            .ocr_metadata(None)
            .quality_recognition(None)
            .build()
    }

    #[test]
    async fn test_create_document() {
        let other_context = OtherContext::new("test".to_string());
        let res_document = create_default_document("test_doc");
        let document = res_document.unwrap();
        let response = other_context
            .create_document(&document, &DocumentType::Document)
            .await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn test_delete_document() {
        let other_context = OtherContext::new("test".to_string());
        let document_name = DOCUMENT_ID;
        let document = create_default_document(document_name).unwrap();
        let _ = other_context
            .create_document(&document, &DocumentType::Document)
            .await;

        let response = other_context
            .delete_document(FOLDER_ID, document_name)
            .await;

        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn test_update_document() {
        let other_context = OtherContext::new("test".to_string());
        let document_name = DOCUMENT_ID;
        let mut document = create_default_document(document_name).unwrap();
        let _ = other_context
            .create_document(&document, &DocumentType::Document)
            .await;

        document.set_doc_path("/new-path");
        let response = other_context.update_document(&document).await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn test_get_document() {
        let other_context = OtherContext::new("test".to_string());
        let document_name = DOCUMENT_ID;
        let document = create_default_document(document_name).unwrap();
        let _ = other_context
            .create_document(&document, &DocumentType::Document)
            .await;

        let response = other_context.get_document(FOLDER_ID, document_name).await;

        assert_eq!(response.unwrap().get_doc_name(), document_name);
    }
}
