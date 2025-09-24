mod api;
mod config;
mod error;
pub mod mw;

pub use config::ServerConfig;

use axum::routing::get;
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use std::sync::Arc;

use crate::application::services::{
    server::ServerApp,
    storage::{DocumentManager, DocumentSearcher, IndexManager, PaginateManager},
};
use crate::application::services::tokenizer::TokenizeProvider;

pub fn init_server<Storage, Searcher, Tokenizer>(app: ServerApp<Storage, Searcher, Tokenizer>) -> Router
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
    Tokenizer: TokenizeProvider + Send + Sync + Clone + 'static,
{
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app_arc = Arc::new(app);
    Router::new()
        .merge(api::init_v1_routers())
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .with_state(app_arc)
}
