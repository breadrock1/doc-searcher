use crate::errors::JsonResponse;
use crate::errors::{ErrorResponse, Successful};
use crate::searcher;
use crate::storage::forms::DocTypeQuery;
use crate::storage::forms::{CreateFolderForm, ShowAllFlag};
use crate::storage::models::Document;
use crate::storage::models::Folder;
use crate::storage::DocumentService;
use crate::storage::FolderService;
use crate::swagger::examples::TestExample;

use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, post, put};
use actix_web::{web, Scope};
use serde_json::Value;

type FolderContext = Data<Box<dyn FolderService>>;
type DocumentContext = Data<Box<dyn DocumentService>>;

pub fn build_scope() -> Scope {
    web::scope("/storage")
        .service(get_folders)
        .service(get_folder)
        .service(create_folder)
        .service(delete_folder)
        .service(get_document)
        .service(create_document)
        .service(delete_document)
        .service(update_document)
        .service(searcher::endpoints::get_index_records)
}

#[utoipa::path(
    get,
    path = "/storage/folders",
    tag = "Folders",
    params(
        (
            "show_all", Query,
            description = "Show all folders",
            example = "true",
        )
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = [Folder],
            example = json!(vec![Folder::test_example(None)]),
        ),
        (
            status = 400,
            description = "Failed while getting all folders",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting folders".to_string(),
                attachments: None,
            }),
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
#[get("/folders")]
async fn get_folders(
    cxt: FolderContext,
    show_all: Query<ShowAllFlag>,
) -> JsonResponse<Vec<Folder>> {
    let client = cxt.get_ref();
    let show_all_flag = show_all.0.flag();
    let folders = client.get_all_folders(show_all_flag).await?;
    Ok(Json(folders))
}

#[utoipa::path(
    get,
    path = "/storage/folders/{folder_id}",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        )
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Folder,
            example = json!(Folder::test_example(None))
        ),
        (
            status = 400,
            description = "Failed while getting folder by id",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting folder by id".to_string(),
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
#[get("/folders/{folder_id}")]
async fn get_folder(cxt: FolderContext, path: Path<String>) -> JsonResponse<Folder> {
    let client = cxt.get_ref();
    let folder = client.get_folder(path.as_ref()).await?;
    Ok(Json(folder))
}

#[utoipa::path(
    put,
    path = "/storage/folders/{folder_id}",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        )
    ),
    request_body(
        content = CreateFolderForm,
        example = json!(CreateFolderForm::test_example(None))
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
                code: 200,
                message: "Done".to_string(),
            }),
        ),
        (
            status = 400,
            description = "Failed while creating new folder",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while creating new folder".to_string(),
                attachments: None,
            }),
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
#[put("/folders/{folder_id}")]
async fn create_folder(
    cxt: FolderContext,
    form: Json<CreateFolderForm>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let folder_form = form.0;
    let status = client.create_folder(&folder_form).await?;
    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/storage/folders/{folder_id}",
    tag = "Folders",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
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
            }),
        ),
        (
            status = 400,
            description = "Failed while deleting folder",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while deleting folder".to_string(),
                attachments: None,
            }),
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
#[delete("/folders/{folder_id}")]
async fn delete_folder(cxt: FolderContext, path: Path<String>) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let folder_id = path.as_str();
    let status = client.delete_folder(folder_id).await?;
    Ok(Json(status))
}

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
    cxt: DocumentContext,
    form: Json<Document>,
    path: Path<(String, String)>,
    document_type: Query<DocTypeQuery>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let doc_form = form.0;
    let (folder_id, _) = path.as_ref();
    let doc_type = document_type.0.get_type();
    let status = client
        .create_document(folder_id.as_str(), &doc_form, &doc_type)
        .await?;
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
    cxt: DocumentContext,
    path: Path<(String, String)>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let (folder_id, doc_id) = path.as_ref();
    let status = client
        .delete_document(folder_id.as_str(), doc_id.as_str())
        .await?;
    Ok(Json(status))
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
    cxt: DocumentContext,
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
    cxt: DocumentContext,
    form: Json<Value>,
    path: Path<(String, String)>,
    document_type: Query<DocTypeQuery>,
) -> JsonResponse<Successful> {
    let client = cxt.get_ref();
    let (folder_id, _) = path.as_ref();
    let doc_type = document_type.0.get_type();
    let status = client
        .update_document(folder_id.as_str(), &form.0, &doc_type)
        .await?;
    Ok(Json(status))
}
