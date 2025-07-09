use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::application::dto::params::{CreateIndexParams, RetrieveDocumentParams};
use crate::application::dto::{Document, Index};
use crate::application::services::server::{ServerError, ServerResult, Success};
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::infrastructure::httpserver::swagger::SwaggerExample;
use crate::infrastructure::httpserver::ServerApp;

pub const STORAGE_ALL_INDEXES_URL: &str = "/storage/indexes";
pub const STORAGE_INDEX_URL: &str = "/storage/{index_id}";
pub const STORAGE_ALL_DOCUMENTS_URL: &str = "/storage/{index_ids}/documents";
pub const STORAGE_DOCUMENT_URL: &str = "/storage/{index_id}/{document_id}";
pub const CREATE_DOCUMENT_URL: &str = "/storage/{index_id}/create";

#[utoipa::path(
    get,
    path = STORAGE_ALL_INDEXES_URL,
    tag = "index",
    description = "Get all existing indexes",
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "List of lll exists indexes ",
            body = Vec<Index>,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to get all indexes",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to get all indexes"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn get_all_indexes<Storage, Searcher>(
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
    path = STORAGE_INDEX_URL,
    tag = "index",
    description = "Get index information by id",
    params(
        (
            "index_id" = &str,
            description = "Index id to get information",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Index information",
            body = Index,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to get index information",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to get index by id"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn get_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let folder = storage.get_index(&index_id).await?;
    Ok(Json(folder))
}

#[utoipa::path(
    put,
    path = STORAGE_INDEX_URL,
    tag = "index",
    description = "Create new index",
    params(
        (
            "index_id" = &str,
            description = "Index id to create",
            example = "test-folder",
        ),
    ),
    request_body(
        content = CreateIndexParams,
    ),
    responses(
        (
            status = 201,
            content_type="application/json",
            description = "Index has been created",
            body = String,
            example = "test-folder"
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to create new index",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to create new index"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn create_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<CreateIndexParams>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let index_id = storage.create_index(&form).await?;
    let status = Success::new(201, &index_id);
    Ok((StatusCode::CREATED, Json(status)))
}

#[utoipa::path(
    delete,
    path = STORAGE_INDEX_URL,
    tag = "index",
    description = "Delete existing index by id",
    params(
        (
            "index_id" = &str,
            description = "Delete existing index by id",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Index has been deleted",
            body = Success
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to delete index",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to delete index"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn delete_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    storage.delete_index(&index_id).await?;
    let status = Success::default();
    Ok(Json(status))
}

#[utoipa::path(
    post,
    path = STORAGE_ALL_DOCUMENTS_URL,
    tag = "document",
    description = "Get all documents stored into index",
    params(
        (
            "index_ids" = &str,
            description = "Index id's to retrieve documents",
            example = "test-folder",
        ),
    ),
    request_body(
        content = RetrieveDocumentParams,
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "List of retrieved documents stored into passed index id",
            body = Vec<Document>,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to retrieve documents stored into index",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to retrieve documents from index"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to delete index"))),
        ),
    )
)]
pub async fn get_documents<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_ids): Path<String>,
    Json(form): Json<RetrieveDocumentParams>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let searcher = state.get_searcher();
    let documents = searcher.retrieve(&index_ids, &form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    get,
    path = STORAGE_DOCUMENT_URL,
    tag = "document",
    description = "Load full Document information by id",
    params(
        (
            "index_id" = &str,
            description = "Index id where is stored Document",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document id to load information",
            example = "cd753717-24cf-4e64-9c51-6dbf3bcb0013",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Document object stored into index",
            body = Document,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to load Document by passed params",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to load document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
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
    path = CREATE_DOCUMENT_URL,
    tag = "document",
    description = "Store new Document to index",
    params(
        (
            "index_id" = &str,
            description = "Index id to store Document object",
            example = "test-folder",
        )
    ),
    request_body(
        content = Document,
    ),
    responses(
        (
            status = 201,
            content_type="application/json",
            description = "Document has been stored successful",
            body = Success,
            example = json!(Success::new(201, "cd753717-24cf-4e64-9c51-6dbf3bcb0013")),
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to store Document object",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to store document"))),
        ),
        (
            status = 503,
            content_type="application/json",
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn store_document<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
    Json(form): Json<Document>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let id = storage.create_document(&index_id, form).await?;

    let status = Success::new(201, &id);
    Ok((StatusCode::CREATED, Json(status)))
}

#[utoipa::path(
    delete,
    path = STORAGE_DOCUMENT_URL,
    tag = "document",
    description = "Delete Document object from index",
    params(
        (
            "index_id" = &str,
            description = "Index id where is stored Document",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document id to delete it",
            example = "cd753717-24cf-4e64-9c51-6dbf3bcb0013",
        ),
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Deleted document by id",
            body = Success,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to delete Document object",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to delete document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
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
    path = STORAGE_DOCUMENT_URL,
    tag = "document",
    description = "Update existing Document object",
    params(
        (
            "index_id" = &str,
            description = "Index id where is stored Document",
            example = "test-folder",
        ),
        (
            "document_id" = &str,
            description = "Document id to update it",
            example = "cd753717-24cf-4e64-9c51-6dbf3bcb0013",
        ),
    ),
    request_body(
        content = Document,
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Document has been updated successful",
            body = Success,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to update Document object",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to update document"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
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
    let (folder_id, doc_id) = path;
    let storage = state.get_storage();
    storage.update_document(&folder_id, &doc_id, form).await?;
    let status = Success::default();
    Ok(Json(status))
}
