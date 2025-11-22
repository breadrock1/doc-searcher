mod api;
mod config;
pub use config::HttpServerConfig;

use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use std::sync::Arc;

use crate::server::ServerApp;

const FILE_BODY_LIMIT_MB: usize = 50;
const BYTE_SIZE: usize = 1024;

pub fn init_server<Storage, Searcher>(app: ServerApp<Storage, Searcher>) -> Router
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone + 'static,
{
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app_arc = Arc::new(app);
    Router::new()
        .merge(api::init_v1_routers())
        // TODO: replace to otlp push model
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .layer(DefaultBodyLimit::disable())
        .layer(DefaultBodyLimit::max(BYTE_SIZE * FILE_BODY_LIMIT_MB))
        .with_state(app_arc)
}
