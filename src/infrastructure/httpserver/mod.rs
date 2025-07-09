mod config;
mod dto;
mod error;
pub mod mw;
mod router;
mod swagger;

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
        .route(
            router::storage::STORAGE_ALL_INDEXES_URL,
            get(router::storage::get_all_indexes),
        )
        .route(
            router::storage::STORAGE_ALL_DOCUMENTS_URL,
            post(router::storage::get_documents),
        )
        .route(
            router::storage::STORAGE_INDEX_URL,
            get(router::storage::get_index)
                .delete(router::storage::delete_index)
                .put(router::storage::create_index),
        )
        .route(
            router::storage::STORAGE_DOCUMENT_URL,
            get(router::storage::get_document)
                .delete(router::storage::delete_document)
                .patch(router::storage::update_document)
                .put(router::storage::store_document),
        )
}

fn init_searcher_layer<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    Router::new()
        .route(
            router::searcher::SEARCH_FULLTEXT_URL,
            post(router::searcher::search_fulltext),
        )
        .route(
            router::searcher::SEARCH_SEMANTIC_URL,
            post(router::searcher::search_semantic),
        )
        .route(
            router::searcher::SEARCH_HYBRID_URL,
            post(router::searcher::search_hybrid),
        )
        .route(
            router::searcher::SEARCH_PAGINATE_URL,
            get(router::searcher::paginate_next).delete(router::searcher::delete_scroll_session),
        )
}
