use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use std::sync::Arc;

use crate::server::{ServerApp, ServerResult};

const HEALTH_URL: &str = "/health";
const METRICS_URL: &str = "/metrics";

pub fn init_system_routers<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    // TODO: replace to otlp push model
    let router: Router<Arc<ServerApp<Storage, Searcher>>> = Router::new()
        .route(HEALTH_URL, get(health))
        .route(METRICS_URL, get(metrics));

    router
}

pub async fn health<Storage, Searcher>(
    State(_state): State<Arc<ServerApp<Storage, Searcher>>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    Ok(StatusCode::OK)
}

pub async fn metrics<Storage, Searcher>(
    State(state): State<Arc<ServerApp<Storage, Searcher>>>,
) -> ServerResult<impl IntoResponse>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let (format_type, body) = state.meter_handle.render_collected_data();
    Ok((
        [(axum::http::header::CONTENT_TYPE, format_type.to_string())],
        body,
    ))
}
