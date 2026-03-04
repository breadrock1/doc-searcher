use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use std::sync::Arc;

use crate::server::{ServerApp, ServerResult};

const HEALTH_URL: &str = "/health";
const METRICS_URL: &str = "/metrics";
const HOME_URL: &str = "/";
const INDEX_HTML_PAGE_DATA: &str = include_str!("../../../../static/index.html");

pub fn init_system_routers<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    // TODO: replace to otlp push model
    let router: Router<Arc<ServerApp<Storage, Searcher>>> = Router::new()
        .route(HOME_URL, get(home))
        .route(HEALTH_URL, get(health))
        .route(METRICS_URL, get(metrics));

    router
}

pub async fn home<Storage, Searcher>(
    State(_state): State<Arc<ServerApp<Storage, Searcher>>>,
) -> Html<String>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    Html(INDEX_HTML_PAGE_DATA.to_owned())
}

pub async fn health<Storage, Searcher>(
    State(_state): State<Arc<ServerApp<Storage, Searcher>>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    Ok(StatusCode::OK)
}

pub async fn metrics<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let (format_type, body) = state.meter_handle.render_collected_data();
    Ok((
        [(axum::http::header::CONTENT_TYPE, format_type.to_string())],
        body,
    ))
}
