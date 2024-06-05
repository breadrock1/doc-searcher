use crate::errors::{ErrorResponse, JsonResponse, Successful, WebError};
use crate::forms::TestExample;
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::{DeleteDocsForm, DocumentType, MoveDocsForm};
use crate::services::searcher::service::DocumentService;

use actix_web::{delete, get, post, put};
use actix_web::web::{Data, Json, Path, Query};
use serde_json::Value;

type Context = Data<Box<dyn DocumentService>>;

#[utoipa::path(
    put,
    path = "/storage/folders/{folder_id}/{document_id}",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document identifier to get",
            example = "98ac9896be35f47fb8442580cd9839b4",
        )
    ),
    request_body(
        content = Document,
        example = json!(Document::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
            })
        )
    )
)]
#[put("/folders/{folder_id}/{document_id}")]
async fn create_document(cxt: Context, form: Json<Document>) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let doc_form = form.0;
    let status = client.create_document(&doc_form).await?;
    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/storage/folders/{folder_id}/{document_id}",
    tag = "Documents",
    params(
        (
            "folder_id" = &str, 
            description = "Folder id where documents is stored",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document identifier to get",
            example = "98ac9896be35f47fb8442580cd9839b4",
        )
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
                message: "Failed while deleting document".to_string(),
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
            })
        )
    )
)]
#[delete("/folders/{folder_id}/{document_id}")]
async fn delete_document(
    cxt: Context,
    path: Path<(String, String)>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let (folder_id, doc_id) = path.as_ref();
    let status = client.delete_document(folder_id.as_str(), doc_id.as_str()).await?;
    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/storage/folders/{folder_id}/documents",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Folder id where documents is stored",
            example = "test-folder",
        )
    ),
    request_body(
        content = DeleteDocsForm,
        example = json!(DeleteDocsForm::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
                message: "Failed while deleting documents: {ids}...".to_string(),
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
            })
        )
    )
)]
#[delete("/folders/{folder_id}/documents")]
async fn delete_documents(
    cxt: Context,
    path: Path<(String, String)>,
    form: Json<DeleteDocsForm>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let document_ids = form.get_doc_ids();
    let (folder_id, _) = path.as_ref();

    let mut failed_tasks = Vec::with_capacity(document_ids.len());
    for id in document_ids.into_iter() {
        let result = client.delete_document(folder_id.as_str(), id).await;
        if result.is_err() {
            failed_tasks.push(id.as_str());
        }
    }

    if !failed_tasks.is_empty() {
        let msg = failed_tasks.join(", ");
        return Err(WebError::DeleteCluster(msg));
    }

    Ok(Json(Successful::success("Done")))
}

#[utoipa::path(
    get,
    path = "/storage/folders/{folder_id}/{document_id}",
    tag = "Documents",
    params(
        (
            "folder_id" = &str, 
            description = "Folder id where document is stored",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document identifier to get",
            example = "<document-id>",
        ),
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
        )
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
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
            })
        )
    )
)]
#[get("/folders/{folder_id}/{document_id}")]
async fn get_document(
    cxt: Context, 
    path: Path<(String, String)>,
    document_type: Query<DocumentType>,
) -> JsonResponse<Value> {
    let client = cxt.get_ref();
    let (folder_id, doc_id) = path.as_ref();
    let document = client.get_document(folder_id.as_str(), doc_id).await?;
    let value = document_type.to_value(&document)?;
    Ok(Json(value))
}

#[utoipa::path(
    post,
    path = "/folders/{folder_id}/move",
    tag = "Documents",
    params(
        (
        "folder_id" = &str,
        description = "Folder id where document is stored",
        example = "test-folder",
        )
    ),
    request_body(
        content = MoveDocsForm,
        example = json!(MoveDocsForm::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
            })
        )
    )
)]
#[post("/folders/{folder_id}/move")]
async fn move_documents(
    cxt: Context,
    path: Path<String>,
    form: Json<MoveDocsForm>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let folder_id = path.as_ref();
    let move_doc_form = form.0;
    let status = client.move_documents(folder_id, &move_doc_form).await?;
    Ok(Json(status))
}

#[utoipa::path(
    post,
    path = "/storage/folders/{folder_id}/{document_id}",
    tag = "Unimplemented",
    params(
        (
            "folder_id" = &str,
            description = "Folder id where document is stored",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document identifier to get",
            example = "98ac9896be35f47fb8442580cd9839b4",
        )
    ),
    request_body(
        content = Document,
        example = json!(Document::test_example(None)),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
            })
        )
    )
)]
#[post("/folders/{folder_id}/{document_id}")]
async fn update_document(
    cxt: Context,
    form: Json<Document>,
    path: Path<(String, String)>,
) -> JsonResponse<Successful> {
    // TODO: Update by UpdateForm to store DocType and Value
    let _client = cxt.get_ref();
    let _doc_form = form.0;
    let (_folder_id, _doc_id) = path.as_ref();
    // let status = client.update_document(folder_id.as_str(), doc_id.as_str(), &doc_form).await?;
    // Ok(web::Json(status))
    Ok(Json(Successful::success("dfg")))
}
