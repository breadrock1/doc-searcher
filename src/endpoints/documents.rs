use crate::errors::{ErrorResponse, JsonResponse, SuccessfulResponse, WebError};
use crate::services::searcher::DocumentsService;

use crate::forms::document::{Document, MoveDocumetsForm};
use crate::forms::TestExample;

use actix_web::{delete, get, post, put, web};
use actix_web::{HttpResponse, ResponseError};

type Context = web::Data<Box<dyn DocumentsService>>;

#[utoipa::path(
    post,
    path = "/documents/create",
    tag = "Documents",
    request_body(
        content = Document,
        example = json!(Document::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while creating document",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while creating document".to_string(),
            })
        ),
    )
)]
#[post("/create")]
async fn create_document(cxt: Context, form: web::Json<Document>) -> HttpResponse {
    let client = cxt.get_ref();
    let doc_form = form.0;
    match client.create_document(&doc_form).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/documents/{folder_id}/{document_ids}",
    tag = "Documents",
    params(
        (
            "folder_id" = &str, 
            description = "Folder id where documents is stored",
            example = "test_folder",
        ),
        (
            "document_ids" = &str, 
            description = "Document identifiers to delete (separate)",
            example = "<document-id-1>,<document-id-2>,...",
        ),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while deleting documents",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while deleting documents".to_string(),
            })
        )
    )
)]
#[delete("/{folder_id}/{document_ids}")]
async fn delete_documents(cxt: Context, path: web::Path<(String, String)>) -> HttpResponse {
    let client = cxt.get_ref();
    let (folder_id, doc_ids) = path.as_ref();

    let documents_id = doc_ids.split(',').collect::<Vec<&str>>();
    let mut failed_tasks: Vec<&str> = Vec::with_capacity(documents_id.len());
    for id in documents_id.into_iter() {
        let result = client.delete_document(folder_id.as_str(), id).await;
        if result.is_err() {
            failed_tasks.push(id);
        }
    }

    if !failed_tasks.is_empty() {
        let msg = failed_tasks.join(",");
        return WebError::DeleteCluster(msg).error_response();
    }

    SuccessfulResponse::ok_response("Done")
}

#[utoipa::path(
    get,
    path = "/documents/{folder_id}/{document_id}",
    tag = "Documents",
    params(
        (
            "folder_id" = &str, 
            description = "Folder id where document is stored",
            example = "test_folder",
        ),
        (
            "document_id" = &str, 
            description = "Document identifier to get",
            example = "<document-id>",
        ),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Document,
            example = json!(Document::test_example(None))
        ),
        (
            status = 400,
            description = "Failed while getting document",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting document".to_string(),
            })
        )
    )
)]
#[get("/{folder_id}/{document_id}")]
async fn get_document(cxt: Context, path: web::Path<(String, String)>) -> JsonResponse<Document> {
    let client = cxt.get_ref();
    let (folder_id, doc_id) = path.as_ref();
    client
        .get_document(folder_id.as_str(), doc_id.as_str())
        .await
}

#[utoipa::path(
    put,
    path = "/documents/{folder_id}/{document_id}",
    tag = "Documents",
    request_body(
        content = Document,
        example = json!(Document::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while updating document",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while updating document".to_string(),
            })
        ),
    )
)]
#[put("/{folder_id}/{document_id}")]
async fn update_document(cxt: Context, form: web::Json<Document>) -> HttpResponse {
    let client = cxt.get_ref();
    let doc_form = form.0;
    match client.update_document(&doc_form).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/documents/location",
    tag = "Documents",
    request_body(
        content = MoveDocumentsForm,
        example = json!(MoveDocumetsForm::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400,
            description = "Failed while moving documents to folder",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while moving documents to folder".to_string(),
            })
        ),
    )
)]
#[post("/location")]
async fn move_documents(cxt: Context, form: web::Json<MoveDocumetsForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let move_doc_form = form.0;
    match client.move_documents(&move_doc_form).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
}

#[cfg(test)]
mod documents_endpoints {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::searcher::DocumentsService;

    use crate::forms::document::{Document, DocumentBuilderError};

    use actix_web::test;

    fn create_default_document(document_name: &str) -> Result<Document, DocumentBuilderError> {
        Document::builder()
            .folder_id("test_folder".to_string())
            .folder_path("/test_folder".to_string())
            .content_uuid("content_uuid".to_string())
            .content_md5("md5 hash".to_string())
            .content("Any document text".to_string())
            .content_vector(Vec::default())
            .document_name(document_name.to_string())
            .document_path(format!("/test_folder/{}", document_name))
            .document_size(1024)
            .document_type("document".to_string())
            .document_extension(".txt".to_string())
            .document_permissions(777)
            .document_md5("md5 hash".to_string())
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
        let response = other_context.create_document(&document).await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn test_delete_document() {
        let other_context = OtherContext::new("test".to_string());
        let document_name = "test_document";
        let document = create_default_document(document_name).unwrap();
        let _ = other_context.create_document(&document).await;

        let response = other_context
            .delete_document("test_folder", document_name)
            .await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn test_update_document() {
        let other_context = OtherContext::new("test".to_string());
        let document_name = "test_document";
        let mut document = create_default_document(document_name).unwrap();
        let _ = other_context.create_document(&document).await;

        document.set_doc_path("/new-path");
        let response = other_context.update_document(&document).await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn test_get_document() {
        let other_context = OtherContext::new("test".to_string());
        let document_name = "test_document";
        let document = create_default_document(document_name).unwrap();
        let _ = other_context.create_document(&document).await;

        let response = other_context
            .get_document("test_folder", document_name)
            .await;
        assert_eq!(response.unwrap().get_doc_name(), document_name);
    }
}
