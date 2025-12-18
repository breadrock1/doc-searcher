use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use std::sync::Arc;
use axum_prometheus::PrometheusMetricLayer;

use crate::server::{ServerApp, ServerResult};

const HEALTH_URL: &str = "/health";

pub fn init_system_routers<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    // TODO: replace to otlp push model
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let router: Router<Arc<ServerApp<Storage, Searcher>>> = Router::new()
        .route(HEALTH_URL, get(health))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);

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
