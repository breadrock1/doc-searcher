use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::application::dto::{Document, Index, RetrieveDocumentParams};
use crate::application::services::server::error::{ServerResult, Success};
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::infrastructure::httpserver::ServerApp;

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
            body = Vec<Index>,
        ),
        (
            status = 400,
            description = "Failed while getting all folders",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn get_folders<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let folders = storage.get_all_indexes().await?;
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
            body = Index,
        ),
        (
            status = 400,
            description = "Failed while getting folder by id",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn get_folder<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let folder = storage.get_index(path.as_ref()).await?;
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
        content = Index,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
        ),
        (
            status = 400,
            description = "Failed while creating new folder",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn create_folder<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<Index>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let status = storage.create_index(form).await?;
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
        ),
        (
            status = 400,
            description = "Failed while deleting folder",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn delete_folder<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    storage.delete_index(&path).await?;
    let status = Success::default();
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
        content = RetrieveDocumentParams,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Vec<Document>,
        ),
        (
            status = 400,
            description = "Failed while getting all records",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn get_documents<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<RetrieveDocumentParams>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let searcher = state.get_searcher();
    let documents = searcher.retrieve(&form).await?;
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
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn get_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let (folder_id, doc_id) = path;
    let storage = state.get_storage();
    let document = storage.get_document(&folder_id, &doc_id).await?;
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
            description = "Created Document",
        ),
        (
            status = 400,
            description = "Failed while creating document",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn create_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
    Json(form): Json<Document>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let (folder_id, _) = path;
    let storage = state.get_storage();
    let document = storage.create_document(&folder_id, form).await?;
    Ok(Json(document))
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
        ),
        (
            status = 400,
            description = "Failed while deleting documents",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn delete_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let (folder_id, doc_id) = path;
    let storage = state.get_storage();
    storage.delete_document(&folder_id, &doc_id).await?;
    let status = Success::default();
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
        ),
        (
            status = 400,
            description = "Failed while updating document",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn update_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(path): Path<(String, String)>,
    Json(form): Json<Document>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let (folder_id, _) = path;
    let storage = state.get_storage();
    storage.update_document(&folder_id, form).await?;
    let status = Success::default();
    Ok(Json(status))
}
