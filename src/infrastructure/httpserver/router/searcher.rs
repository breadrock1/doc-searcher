use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::application::dto::{
    Document, FullTextSearchParams, PaginateParams, SemanticSearchParams,
};
use crate::application::services::server::error::{ServerResult, Success};
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::infrastructure::httpserver::ServerApp;

#[utoipa::path(
    post,
    path = "/search/fulltext",
    tag = "searcher",
    description = "Fulltext searching",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = FullTextSearchParams,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Vec<Document>,
        ),
        (
            status = 400,
            description = "Failed while searching documents",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn search_fulltext<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<FullTextSearchParams>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let searcher = state.get_searcher();
    let documents = searcher.fulltext(&form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/search/semantic",
    tag = "searcher",
    description = "Semantic search by vector",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = SemanticSearchParams,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Vec<Document>,
        ),
        (
            status = 400,
            description = "Failed while searching tokens",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn search_semantic<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<SemanticSearchParams>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let searcher = state.get_searcher();
    let documents = searcher.semantic(&form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    post,
    path = "/search/paginate/next",
    description = "Load next chunk of search results",
    tag = "searcher",
    params(
        (
            "document_type", Query,
            description = "Document type to convert",
            example = "document",
        ),
    ),
    request_body(
        content = PaginateParams,
    ),
    responses(
        (
            status = 200,
            description = "Successful",
            body = Vec<Document>,
        ),
        (
            status = 400,
            description = "Failed while scrolling",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn paginate_next<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<PaginateParams>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let searcher = state.get_searcher();
    let documents = searcher.paginate(&form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    delete,
    path = "/search/paginate/{session_id}",
    tag = "searcher",
    description = "Delete all existing pagination sessions",
    responses(
        (
            status = 200,
            description = "Successful",
        ),
        (
            status = 400,
            description = "Failed to delete paginate session",
        ),
        (
            status = 503,
            description = "Server does not available",
        ),
    )
)]
pub async fn delete_scrolls<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Path(session_id): Path<String>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let searcher = state.get_searcher();
    searcher.delete_session(&session_id).await?;
    let status = Success::default();
    Ok(Json(status))
}
