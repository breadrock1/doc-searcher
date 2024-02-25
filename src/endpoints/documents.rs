use crate::endpoints::ContextData;
use crate::errors::JsonResponse;

use wrappers::document::Document;

use actix_web::{delete, get, post, put, web, HttpResponse};

#[utoipa::path(
    put,
    path = "/document/update",
    tag = "Update stored Document with passed data",
    request_body = Document,
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 401, description = "Failed while updating document", body = ErrorResponse),
    )
)]
#[put("/update")]
async fn update_document(cxt: ContextData, form: web::Json<Document>) -> HttpResponse {
    let client = cxt.get_ref();
    let doc_form = form.0;
    client.update_document(&doc_form).await
}

#[utoipa::path(
    post,
    path = "/document/new",
    tag = "Create new Document with passed data",
    request_body = Document,
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 401, description = "Failed while creating document", body = ErrorResponse),
    )
)]
#[post("/new")]
async fn new_document(cxt: ContextData, form: web::Json<Document>) -> HttpResponse {
    let client = cxt.get_ref();
    let doc_form = form.0;
    client.create_document(&doc_form).await
}

#[utoipa::path(
    delete,
    path = "/document/{bucket_name}/{document_id}",
    tag = "Delete stored document by document id",
    params(
        ("bucket_name" = &str, description = "Bucket name where document is stored"),
        ("document_id" = &str, description = "Document identifier to delete"),
    ),
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 401, description = "Failed while deleting document", body = ErrorResponse),
    )
)]
#[delete("/{bucket_name}/{document_id}")]
async fn delete_document(cxt: ContextData, path: web::Path<(String, String)>) -> HttpResponse {
    let client = cxt.get_ref();
    let (bucket_name, doc_id) = path.as_ref();
    client
        .delete_document(bucket_name.as_str(), doc_id.as_str())
        .await
}

#[utoipa::path(
    get,
    path = "/document/{bucket_name}/{document_id}",
    tag = "Get stored document by document id",
    params(
        ("bucket_name" = &str, description = "Bucket name where document is stored"),
        ("document_id" = &str, description = "Document identifier to get"),
    ),
    responses(
        (status = 200, description = "Successful", body = Document),
        (status = 401, description = "Failed while getting document", body = ErrorResponse),
    )
)]
#[get("/{bucket_name}/{document_id}")]
async fn get_document(
    cxt: ContextData,
    path: web::Path<(String, String)>,
) -> JsonResponse<Document> {
    let client = cxt.get_ref();
    let (bucket_name, doc_id) = path.as_ref();
    client
        .get_document(bucket_name.as_str(), doc_id.as_str())
        .await
}

#[cfg(test)]
mod documents_endpoints {
    use crate::service::own_engine::context::OtherContext;
    use crate::service::ServiceClient;

    use wrappers::document::{Document, DocumentBuilder, DocumentBuilderError};

    use actix_web::test;

    fn create_default_document(document_name: &str) -> Result<Document, DocumentBuilderError> {
        DocumentBuilder::default()
            .bucket_uuid("test_bucket".to_string())
            .bucket_path("/".to_string())
            .content_uuid("content_uuid".to_string())
            .content_md5("md5 hash".to_string())
            .content("Any document text".to_string())
            .content_vector(Vec::default())
            .document_name(document_name.to_string())
            .document_path("/".to_string())
            .document_size(1024)
            .document_type("document".to_string())
            .document_extension(".txt".to_string())
            .document_permissions(777)
            .document_md5("md5 hash".to_string())
            .document_ssdeep("ssdeep hash".to_string())
            .document_created(None)
            .document_modified(None)
            .highlight(None)
            .build()
    }

    #[test]
    async fn test_create_document() {
        let other_context = OtherContext::_new("test".to_string());
        let res_document = create_default_document("test_doc");
        let document = res_document.unwrap();
        let response = other_context.create_document(&document).await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_delete_document() {
        let other_context = OtherContext::_new("test".to_string());
        let document_name = "test_document";
        let document = create_default_document(document_name).unwrap();
        let _ = other_context.create_document(&document).await;

        let response = other_context
            .delete_document("test_bucket", document_name)
            .await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_update_document() {
        let other_context = OtherContext::_new("test".to_string());
        let document_name = "test_document";
        let mut document = create_default_document(document_name).unwrap();
        let _ = other_context.create_document(&document).await;

        document.document_path = "/new-path".to_string();
        let response = other_context.update_document(&document).await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_get_document() {
        let other_context = OtherContext::_new("test".to_string());
        let document_name = "test_document";
        let document = create_default_document(document_name).unwrap();
        let _ = other_context.create_document(&document).await;

        let response = other_context
            .get_document("test_bucket", document_name)
            .await;
        assert_eq!(response.unwrap().document_name.as_str(), document_name);
    }
}
