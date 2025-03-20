use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;
use std::sync::Arc;

use crate::engine::{DocumentService, FolderService, PaginatorService, SearcherService};
use crate::engine::form::{CreateFolderForm, FolderTypeQuery, RetrieveParams, ShowAllFlag};
use crate::engine::model::{Document, Folder, FolderType, Paginated};
use crate::errors::{ErrorResponse, ServerResult, Successful};
use crate::server::swagger::SwaggerExample;
use crate::server::ServerApp;
use crate::tokenizer::TokenizerService;

#[utoipa::path(
    get,
    path = "/storage/folders",
    tag = "folders",
    description = "Get all existing folders",
    params(
        (
            "show_all", Query,
            description = "Show all folders",
            example = "true",
        ),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Vec<Folder>,
        ),
        (
            status = 400,
            description = "Failed while getting all folders",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while getting all folders"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn get_folders<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Query(show_all): Query<ShowAllFlag>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let show_all_flag = show_all.show_all();
    let folders = state.folders.get_folders(show_all_flag).await?;
    Ok(Json(folders))
}

#[utoipa::path(
    get,
    path = "/storage/{folder_id}",
    tag = "folders",
    description = "Get folder info by folder id",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 200,
            description = "Returned Folder structure",
            body = Folder,
        ),
        (
            status = 400,
            description = "Failed while getting folder by id",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while getting folder by id"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn get_folder<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Path(path): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let folder = state.folders.get_folder(path.as_ref()).await?;
    Ok(Json(folder))
}

#[utoipa::path(
    put,
    path = "/storage/{folder_id}",
    tag = "folders",
    description = "Create new folder",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        ),
    ),
    request_body(
        content = CreateFolderForm,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
        ),
        (
            status = 400,
            description = "Failed while creating new folder",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while creating new folder"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn create_folder<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Json(form): Json<CreateFolderForm>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let status = state.folders.create_folder(&form).await?;
    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/storage/{folder_id}",
    tag = "folders",
    description = "Delete folder by folder id",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
        ),
        (
            status = 400,
            description = "Failed while deleting folder",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while deleting folder"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn delete_folder<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Path(path): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let status = state.folders.delete_folder(&path).await?;
    Ok(Json(status))
}

#[utoipa::path(
    post,
    path = "/storage/{folder_id}/documents",
    tag = "documents",
    description = "Get documents from folder",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get stored documents",
            example = "test-folder",
        ),
        (
            "folder_type", Query,
            description = "Folder type to retrieve",
            example = "document",
        ),
    ),
    request_body(
        content = RetrieveParams,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            // body = Paginated<Vec<Document>>,
        ),
        (
            status = 400,
            description = "Failed while getting all records",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while getting all records"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn get_documents<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    // Query(folder_type): Query<FolderTypeQuery>,
    // Json(form): Json<RetrieveParams>,
    Path(path): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    // let folder_type = folder_type.folder_type();
    let folder_type = FolderType::Document;
    let form = RetrieveParams::default();
    let documents = state.documents.get_documents(&path, &folder_type, &form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    get,
    path = "/storage/{folder_id}/{document_id}",
    tag = "documents",
    description = "Get document object by folder and document ids",
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
            "folder_type", Query,
            description = "Folder type to get",
            example = "document",
        ),
    ),
    responses(
        (
            status = 200,
            description = "Returned document by id",
            body = Document,
        ),
        (
            status = 400,
            description = "Failed while getting document",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while getting document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn get_document<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Query(folder_type): Query<FolderTypeQuery>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let (folder_id, doc_id) = path;
    let folder_type = folder_type.folder_type();
    let document = state.documents.get_document(&folder_id, &doc_id, &folder_type).await?;
    Ok(Json(document))
}

#[utoipa::path(
    put,
    path = "/storage/{folder_id}/{document_id}",
    tag = "documents",
    description = "Create and store new document",
    params(
        (
            "folder_id" = &str,
            description = "Passed folder id to get details",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document identifier to get",
            example = "<document-id>",
        ),
        (
            "folder_type", Query,
            description = "Folder type to create document",
            example = "document",
        ),
    ),
    request_body(
        content = Document,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
        ),
        (
            status = 400,
            description = "Failed while creating document",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while creating document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn create_document<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Query(folder_type): Query<FolderTypeQuery>,
    Path(path): Path<(String, String)>,
    Json(form): Json<Document>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let (folder_id, _) = path;
    let folder_type = folder_type.folder_type();
    let status = state.documents.create_document(&folder_id, &form, &folder_type).await?;
    Ok(Json(status))
}

#[utoipa::path(
    delete,
    path = "/storage/{folder_id}/{document_id}",
    tag = "documents",
    description = "Delete document",
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
        ),
    ),
    responses(
        (
            status = 200,
            description = "Deleted document by id",
            body = Successful,
        ),
        (
            status = 400,
            description = "Failed while deleting documents",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while deleting documents"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn delete_document<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let (folder_id, doc_id) = path;
    let status = state.documents.delete_document(&folder_id, &doc_id).await?;
    Ok(Json(status))
}

#[utoipa::path(
    patch,
    path = "/storage/{folder_id}/{document_id}",
    tag = "documents",
    description = "Updated stored document object",
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
            "folder_type", Query,
            description = "Folder type to update document",
            example = "document",
        ),
    ),
    request_body(
        content = Document,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
        ),
        (
            status = 400,
            description = "Failed while updating document",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(Some("Failed while updating document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::example(None)),
        ),
    )
)]
pub async fn update_document<F, D, S, P, T>(
    State(state): State<Arc<ServerApp<F, D, S, P, T>>>,
    Query(folder_type): Query<FolderTypeQuery>,
    Path(path): Path<(String, String)>,
    Json(form): Json<Value>,
) -> ServerResult<impl IntoResponse>
where
    F: FolderService + Send + Sync,
    D: DocumentService + Send + Sync,
    S: SearcherService + Send + Sync,
    P: PaginatorService + Send + Sync,
    T: TokenizerService + Send + Sync,
{
    let (folder_id, _) = path;
    let folder_type = folder_type.folder_type();
    let status = state.documents.update_document(&folder_id, &form, &folder_type).await?;
    Ok(Json(status))
}
