mod api;
mod config;
pub use config::HttpServerConfig;
pub mod mw;
mod swagger;

use axum::extract::DefaultBodyLimit;
use axum::Router;
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

    let app_arc = Arc::new(app);
    Router::new()
        .merge(api::v1::init_routers())
        .merge(api::system::init_system_routers())
        .merge(swagger::init_swagger_layer())
        .layer(DefaultBodyLimit::disable())
        .layer(DefaultBodyLimit::max(BYTE_SIZE * FILE_BODY_LIMIT_MB))
        .with_state(app_arc)
}
