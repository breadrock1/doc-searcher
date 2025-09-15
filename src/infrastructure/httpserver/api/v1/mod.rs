mod models;
mod router;
mod swagger;

use axum::routing::{get, post, put};
use axum::Router;
use std::sync::Arc;

use crate::application::services::server::ServerApp;
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};

const API_VERSION: &str = "v1";

pub fn init_v1_routers<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone + 'static,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone + 'static,
{
    let router: Router<Arc<ServerApp<Storage, Searcher>>> = Router::new()
        .merge(swagger::init_swagger_layer(API_VERSION))
        .merge(init_storage_layer())
        .merge(init_searcher_layer());

    router
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
            post(router::storage::get_documents).put(router::storage::store_documents),
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
                .patch(router::storage::update_document),
        )
        .route(
            router::storage::CREATE_DOCUMENT_URL,
            put(router::storage::store_document),
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
