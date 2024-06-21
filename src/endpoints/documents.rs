use crate::errors::{ErrorResponse, JsonResponse, Successful, WebError, WebErrorEntity};
use crate::forms::TestExample;
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::{DeleteDocsForm, DocTypeQuery, MoveDocsForm};
use crate::services::searcher::service::DocumentService;

use actix_web::{delete, get, post, put};
use actix_web::web::{Data, Json, Path, Query};
use serde_json::Value;

type Context = Data<Box<dyn DocumentService>>;

#[utoipa::path(
    put,
    path = "/storage/folders/{folder_id}/documents/{document_id}",
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
        ),
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
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
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[put("/folders/{folder_id}/documents/{document_id}")]
async fn create_document(
    cxt: Context,
    form: Json<Document>,
    path: Path<(String, String)>,
    document_type: Query<DocTypeQuery>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let doc_form = form.0;
    let (folder_id, _) = path.as_ref();
    let doc_type = document_type.0.get_type();
    let status = client.create_document(folder_id.as_str(), &doc_form, &doc_type).await?;
    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/storage/folders/{folder_id}/documents/{document_id}",
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
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[delete("/folders/{folder_id}/documents/{document_id}")]
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
        content = Vec<DeleteDocsForm>,
        example = json!(vec![DeleteDocsForm::test_example(None)])
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
                attachments: Some(vec!["98ac9896be35f47fb8442580cd9839b4".to_string()]),
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
                attachments: None,
            })
        )
    )
)]
#[delete("/folders/{folder_id}/documents")]
async fn delete_documents(
    cxt: Context,
    _path: Path<String>,
    form: Json<Vec<DeleteDocsForm>>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let mut failed_tasks = Vec::new();
    for doc_form in form.0.iter() {
        let folder_id = doc_form.get_folder_id();
        for id in doc_form.get_doc_ids() {
            let result = client.delete_document(folder_id, id).await;
            if result.is_err() {
                failed_tasks.push(id.to_owned());
            }
        }
    }

    if !failed_tasks.is_empty() {
        let msg = "Not deleted".to_string();
        let entity = WebErrorEntity::with_attachments(msg, failed_tasks);
        return Err(WebError::DeleteDocument(entity));
    }

    Ok(Json(Successful::success("Done")))
}

#[utoipa::path(
    get,
    path = "/storage/folders/{folder_id}/documents/{document_id}",
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
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[get("/folders/{folder_id}/documents/{document_id}")]
async fn get_document(
    cxt: Context, 
    path: Path<(String, String)>,
    document_type: Query<DocTypeQuery>,
) -> JsonResponse<Value> {
    let client = cxt.get_ref();
    let (folder_id, doc_id) = path.as_ref();
    let document = client.get_document(folder_id.as_str(), doc_id).await?;
    let doc_type = document_type.0.get_type();
    let value = doc_type.to_value(&document)?;
    Ok(Json(value))
}

#[utoipa::path(
    post,
    path = "/storage/folders/{folder_id}/move",
    tag = "Documents",
    params(
        (
            "folder_id" = &str,
            description = "Folder id where document is stored",
            example = "test-folder",
        )
    ),
    request_body(
        content = Vec<MoveDocsForm>,
        example = json!(vec![MoveDocsForm::test_example(None)]),
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
                attachments: Some(vec!["98ac9896be35f47fb8442580cd9839b4".to_string()]),
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
                attachments: None,
            })
        )
    )
)]
#[post("/folders/{folder_id}/move")]
async fn move_documents(
    cxt: Context,
    _path: Path<String>,
    form: Json<Vec<MoveDocsForm>>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let mut failed_tasks = Vec::new();
    for doc_form in form.0.iter() {
        let folder_id = doc_form.get_folder_id();
        let result = client.move_documents(folder_id, doc_form).await;
        if result.is_err() {
            let err = result.err().unwrap();
            let ids = err.attachments().unwrap_or_default();
            failed_tasks.extend_from_slice(ids.as_slice());
        }
    }

    if !failed_tasks.is_empty() {
        let msg = "Not deleted".to_string();
        let entity = WebErrorEntity::with_attachments(msg, failed_tasks);
        return Err(WebError::DeleteDocument(entity));
    }

    Ok(Json(Successful::success("Done")))
}

#[utoipa::path(
    post,
    path = "/storage/folders/{folder_id}/documents/{document_id}",
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
            example = "98ac9896be35f47fb8442580cd9839b4",
        ),
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document"
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
                attachments: None,
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
                attachments: None,
            })
        )
    )
)]
#[post("/folders/{folder_id}/documents/{document_id}")]
async fn update_document(
    cxt: Context,
    form: Json<Value>,
    path: Path<(String, String)>,
    document_type: Query<DocTypeQuery>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let (folder_id, _) = path.as_ref();
    let doc_type = document_type.0.get_type();
    let status = client.update_document(
        folder_id.as_str(),
        &form.0,
        &doc_type
    )
    .await?;
    Ok(Json(status))
}
