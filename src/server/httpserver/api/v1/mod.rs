#[cfg(test)]
pub mod tests;

pub mod form;
mod query;
pub mod router;
pub mod schema;

use axum::routing::{get, post, put};
use axum::Router;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use std::sync::Arc;

use crate::server::ServerApp;

pub const API_VERSION_URL: &str = "/api/v1";

pub fn init_routers<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    let router: Router<Arc<ServerApp<Storage, Searcher>>> = Router::new()
        .nest(API_VERSION_URL, init_storage_layer())
        .nest(API_VERSION_URL, init_searcher_layer());

    router
}

fn init_storage_layer<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
{
    Router::new()
        .route(
            router::index::STORAGE_ALL_INDEXES_URL,
            get(router::index::get_all_indexes),
        )
        .route(
            router::index::STORAGE_INDEX_URL,
            get(router::index::get_index)
                .delete(router::index::delete_index)
                .put(router::index::create_index),
        )
        .route(
            router::document::STORAGE_ALL_DOCUMENTS_URL,
            post(router::document::get_index_documents).put(router::document::store_documents),
        )
        .route(
            router::document::STORAGE_DOCUMENT_URL,
            get(router::document::get_document_parts).delete(router::document::delete_document),
        )
        .route(
            router::document::CREATE_DOCUMENT_URL,
            put(router::document::store_document),
        )
}

fn init_searcher_layer<Storage, Searcher>() -> Router<Arc<ServerApp<Storage, Searcher>>>
where
    Searcher: ISearcher + IPaginator + Send + Sync + 'static,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + 'static,
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
            get(router::searcher::paginate_next),
        )
}
