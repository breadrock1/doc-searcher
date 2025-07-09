use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::application::dto::params::{
    FullTextSearchParams, HybridSearchParams, PaginateParams, SemanticSearchParams,
};
use crate::application::dto::{Document, Paginated};
use crate::application::services::server::{ServerError, ServerResult, Success};
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::infrastructure::httpserver::dto::PaginateQuery;
use crate::infrastructure::httpserver::swagger::SwaggerExample;
use crate::infrastructure::httpserver::ServerApp;

pub const SEARCH_FULLTEXT_URL: &str = "/search/fulltext";
pub const SEARCH_SEMANTIC_URL: &str = "/search/semantic";
pub const SEARCH_HYBRID_URL: &str = "/search/hybrid";
pub const SEARCH_PAGINATE_URL: &str = "/search/paginate/{session_id}";

#[utoipa::path(
    post,
    path = SEARCH_FULLTEXT_URL,
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
    path = SEARCH_SEMANTIC_URL,
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
    path = SEARCH_HYBRID_URL,
    tag = "search",
    description = "Search Document objects by hybrid algorithm",
    request_body(
        content = HybridSearchParams,
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
pub async fn search_hybrid<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<HybridSearchParams>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let searcher = state.get_searcher();
    let documents = searcher.hybrid(&form).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    get,
    path = SEARCH_PAGINATE_URL,
    tag = "search",
    description = "Paginate search results by scroll",
    params(
        (
            "session_id" = &str,
            description = "Sessions id of scroll to get next paginated result",
            example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk",
        ),
        (
            "lifetime" = &str,
            Query,
            description = "Lifetime of scroll before it will be deleted",
            example = "5m",
        ),
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
    Path(path): Path<String>,
    Query(query): Query<PaginateQuery>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let lifetime = query.lifetime();
    let params = PaginateParams::builder()
        .lifetime(lifetime)
        .scroll_id(path)
        .build()
        .unwrap();

    let searcher = state.get_searcher();
    let documents = searcher.paginate(&params).await?;
    Ok(Json(documents))
}

#[utoipa::path(
    delete,
    path = SEARCH_PAGINATE_URL,
    tag = "search",
    description = "Delete existing pagination session by id",
    params(
        (
            "session_id" = &str,
            description = "Session id of scroll to delete",
            example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk",
        ),
    ),
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
