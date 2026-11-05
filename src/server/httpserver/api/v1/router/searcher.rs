use axum::extract::State;
use axum::Json;
use doc_search_core::domain::searcher::models::PaginationParams;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use std::sync::Arc;

use crate::server::httpserver::api::v1::form::{
    FullTextSearchForm, HybridSearchForm, PaginateForm, SemanticSearchForm,
};
use crate::server::httpserver::api::v1::schema::PaginationSchema;
use crate::server::httpserver::swagger::DefaultErrorForm;
use crate::server::httpserver::ServerApp;
use crate::server::ServerResult;

pub const SEARCH_FULLTEXT_URL: &str = "/search/fulltext";
pub const SEARCH_SEMANTIC_URL: &str = "/search/semantic";
pub const SEARCH_HYBRID_URL: &str = "/search/hybrid";
pub const SEARCH_PAGINATE_URL: &str = "/search/paginate";

const FULLTEXT_DESCRIPTION: &str = include_str!("../../../swagger/descriptions/searcher-fulltext");
const SEMANTIC_DESCRIPTION: &str = include_str!("../../../swagger/descriptions/searcher-semantic");
const HYBRID_DESCRIPTION: &str = include_str!("../../../swagger/descriptions/searcher-hybrid");

#[utoipa::path(
    post,
    tag = "search",
    path = SEARCH_FULLTEXT_URL,
    description = FULLTEXT_DESCRIPTION,
    request_body(content = FullTextSearchForm),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Paginate structure with list of founded Documents",
            body = PaginationSchema,
        ),
        (status = 400, description = "Failed while fulltext searching"),
        (status = 401, description = "Unauthorized access"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn search_fulltext<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<FullTextSearchForm>,
) -> ServerResult<Json<PaginationSchema>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let params = form.try_into()?;
    let searcher = state.get_searcher();
    let pagination = searcher.search_document_parts(params).await?;
    let response = pagination.try_into()?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    tag = "search",
    path = SEARCH_SEMANTIC_URL,
    description = SEMANTIC_DESCRIPTION,
    request_body(content = SemanticSearchForm),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Paginate structure with list of founded Documents",
            body = PaginationSchema,
        ),
        (status = 400, description = "Failed while semantic searching"),
        (status = 401, description = "Unauthorized access"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn search_semantic<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<SemanticSearchForm>,
) -> ServerResult<Json<PaginationSchema>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let params = form.try_into()?;
    let searcher = state.get_searcher();
    let pagination = searcher.search_document_parts(params).await?;
    let response = pagination.try_into()?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    tag = "search",
    path = SEARCH_HYBRID_URL,
    description = HYBRID_DESCRIPTION,
    request_body(content = HybridSearchForm),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Paginate structure with list of founded Documents",
            body = PaginationSchema,
        ),
        (status = 400, description = "Failed while hybrid searching"),
        (status = 401, description = "Unauthorized access"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn search_hybrid<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<HybridSearchForm>,
) -> ServerResult<Json<PaginationSchema>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let params = form.try_into()?;
    let searcher = state.get_searcher();
    let pagination = searcher.search_document_parts(params).await?;
    let response = pagination.try_into()?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    tag = "search",
    path = SEARCH_PAGINATE_URL,
    description = "Paginate search results by scroll. Pass the original search query and `is_reranker_enable` to rerank the next page.",
    request_body(content = PaginateForm),
    responses(
        (
            status = 200,
            content_type="application/json",
            description = "Paginate structure with list of founded Documents",
            body = PaginationSchema,
        ),
        (status = 400, description = "Failed while paginate searching result"),
        (status = 401, description = "Unauthorized access"),
        (status = 500, description = "Internal error"),
        (status = 501, description = "Error form", body = DefaultErrorForm),
    )
)]
pub async fn paginate_next<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
    Json(form): Json<PaginateForm>,
) -> ServerResult<Json<PaginationSchema>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let params = PaginationParams::try_from(form)?;

    let searcher = state.get_searcher();
    let documents = searcher.paginate(params).await?;
    let response = documents.try_into()?;
    Ok(Json(response))
}
