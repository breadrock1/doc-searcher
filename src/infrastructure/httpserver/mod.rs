mod config;
mod dto;
mod error;
mod router;
mod swagger;
pub mod mw;

pub use config::ServerConfig;

use axum::routing::{get, post};
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use std::sync::Arc;

use crate::application::services::{
    server::ServerApp,
    storage::{DocumentManager, DocumentSearcher, IndexManager, PaginateManager},
};

const SWAGGER_CONFIG_FILE: &str = "/api-docs/openapi.json";

pub fn init_server<Storage, Searcher>(app: ServerApp<Storage, Searcher>) -> Router
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let swagger_layer = swagger::init_swagger_layer();
    let storage_layer = init_storage_layer();
    let searcher_layer = init_searcher_layer();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app_arc = Arc::new(app);
    Router::new()
        .merge(swagger_layer)
        .merge(storage_layer)
        .merge(searcher_layer)
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .with_state(app_arc)
}

fn init_storage_layer<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    Router::new()
        .route("/storage/index", get(router::storage::get_all_indexes))
        .route(
            "/storage/{index_id}/documents",
            post(router::storage::get_documents),
        )
        .route(
            "/storage/{index_id}",
            get(router::storage::get_index)
                .delete(router::storage::delete_index)
                .post(router::storage::create_index),
        )
        .route(
            "/storage/{index_id}/{document_id}",
            get(router::storage::get_document)
                .delete(router::storage::delete_document)
                .patch(router::storage::update_document)
                .post(router::storage::store_document),
        )
}

fn init_searcher_layer<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    Router::new()
        .route("/search/fulltext", post(router::searcher::search_fulltext))
        .route("/search/semantic", post(router::searcher::search_semantic))
        .route("/search/paginate", post(router::searcher::paginate_next))
        .route(
            "/search/paginate/{session_id}",
            get(router::searcher::paginate_next)
                .delete(router::searcher::delete_scroll_session),
        )
}
