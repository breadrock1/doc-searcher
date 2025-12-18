use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use std::sync::Arc;

use crate::server::httpserver::api::v1::form::CreateIndexForm;
use crate::server::httpserver::api::v1::schema::IndexSchema;
use crate::server::httpserver::swagger::DefaultErrorForm;
use crate::server::httpserver::ServerApp;
use crate::server::{ServerResult, Success};

pub const STORAGE_ALL_INDEXES_URL: &str = "/storage/indexes";
pub const STORAGE_INDEX_URL: &str = "/storage/{index_id}";

#[utoipa::path(
    get,
    tag = "index",
    path = STORAGE_ALL_INDEXES_URL,
    description = "Get all existing indexes",
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "List of all exists indexes",
            body = Vec<IndexSchema>,
        ),
        (status = 400, description = "Failed to get all indexes"),
        (status = 401, description = "Unauthorized access"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_all_indexes<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let indexes = storage.get_all_indexes().await?;

    let indexes_schema = indexes
        .into_iter()
        .map(|it| it.into())
        .collect::<Vec<IndexSchema>>();

    Ok(Json(indexes_schema))
}

#[utoipa::path(
    get,
    tag = "index",
    path = STORAGE_INDEX_URL,
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
            body = IndexSchema,
        ),
        (status = 400, description = "Failed to get index information"),
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Index not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn get_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    let folder = storage.get_index(&index_id).await?;
    Ok(Json(folder))
}

#[utoipa::path(
    put,
    tag = "index",
    path = STORAGE_INDEX_URL,
    description = "Create new index",
    request_body(content = CreateIndexForm),
    params(
        (
            "index_id" = &str,
            description = "Index id to create",
            example = "test-folder",
        ),
    ),
    responses(
        (
            status = 201,
            content_type="application/json",
            description = "Index has been created",
            body = String,
            example = "test-folder"
        ),
        (status = 400, description = "Validation form error"),
        (status = 401, description = "Unauthorized access"),
        (status = 409, description = "Conflict while creating index"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn create_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(_index_id): Path<String>,
    Json(form): Json<CreateIndexForm>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let params = form.try_into()?;
    let storage = state.get_storage();
    let index_id = storage.create_index(&params).await?;
    let status = Success::new(201, &index_id);
    Ok((StatusCode::CREATED, Json(status)))
}

#[utoipa::path(
    delete,
    tag = "index",
    path = STORAGE_INDEX_URL,
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
        (status = 401, description = "Unauthorized access"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn delete_index<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(index_id): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let storage = state.get_storage();
    storage.delete_index(&index_id).await?;
    let status = Success::default();
    Ok(Json(status))
}
