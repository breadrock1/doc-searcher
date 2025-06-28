use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::application::dto::{Document, FullTextSearchParams, PaginateParams, Paginated, SemanticSearchParams};
use crate::application::services::server::{ServerError, ServerResult, Success};
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::infrastructure::httpserver::ServerApp;
use crate::infrastructure::httpserver::swagger::SwaggerExample;

#[utoipa::path(
    post,
    path = "/search/fulltext",
    tag = "search",
    description = "Search Document objects by fulltext algorithm",
    request_body(
        content = FullTextSearchParams,
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Paginate structure with list of founded Documents",
            body = Paginated<Document>,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed while searching documents",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to found documents"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
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
    tag = "search",
    description = "Search Document objects by semantic algorithm",
    request_body(
        content = SemanticSearchParams,
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Paginate structure with list of founded Documents",
            body = Paginated<Document>,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed while searching documents",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to found documents"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
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
    path = "/search/paginate",
    tag = "search",
    description = "Paginate search results by scroll",
    request_body(
        content = PaginateParams,
    ),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Paginate structure with list of founded Documents",
            body = Paginated<Document>,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed while paginate search result",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to paginate search result"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
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
    tag = "search",
    description = "Delete existing pagination session by id",
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Successful",
            body = Success,
        ),
        (
            status = 400,
            content_type="application/json",
            description = "Failed to delete scroll session",
            body = ServerError,
            example = json!(ServerError::example(Some("failed to delete scroll session"))),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ServerError,
            example = json!(ServerError::example(None)),
        ),
    )
)]
pub async fn delete_scroll_session<Storage, Searcher>(
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
