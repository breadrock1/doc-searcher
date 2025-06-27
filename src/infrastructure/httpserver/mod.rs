pub mod config;
mod error;
mod router;
pub(crate) mod swagger;

use axum::routing::{get, post};
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::application::services::{
    server::ServerApp,
    storage::{DocumentManager, DocumentSearcher, IndexManager, PaginateManager},
};
use crate::infrastructure::httpserver::swagger::ApiDoc;

pub fn init_server<Storage, Searcher>(app: ServerApp<Storage, Searcher>) -> Router
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app_arc = Arc::new(app);
    Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc"))
        .route("/storage/folders", get(router::storage::get_all_indexes))
        .route(
            "/storage/{folder_id}",
            get(router::storage::get_index)
                .delete(router::storage::delete_index)
                .put(router::storage::create_index),
        )
        .route(
            "/storage/{folder_id}/documents",
            post(router::storage::get_documents),
        )
        .route(
            "/storage/{folder_id}/{document_id}",
            get(router::storage::get_document)
                .delete(router::storage::delete_document)
                .patch(router::storage::update_document)
                .put(router::storage::store_document),
        )
        .route("/search/fulltext", post(router::searcher::search_fulltext))
        .route("/search/semantic", post(router::searcher::search_semantic))
        .route(
            "/search/paginate/next",
            post(router::searcher::paginate_next),
        )
        .route(
            "/search/paginate/sessions",
            post(router::searcher::delete_scroll_session),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .with_state(app_arc)
}
