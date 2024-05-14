use crate::errors::{ErrorResponse, JsonResponse, SuccessfulResponse, WebError};
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::MoveDocumentsForm;
use crate::forms::TestExample;
use crate::services::service::DocumentsService;

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
        example = json!(MoveDocumentsForm::test_example(None)),
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
async fn move_documents(cxt: Context, form: web::Json<MoveDocumentsForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let move_doc_form = form.0;
    match client.move_documents(&move_doc_form).await {
        Ok(response) => response.to_response(),
        Err(err) => err.error_response(),
    }
}
