pub mod config;
mod errors;
mod router;
pub(crate) mod swagger;

use crate::engine::{DocumentService, FolderService, PaginatorService, SearcherService};
use crate::tokenizer::TokenizerService;
use axum::routing::{get, post};
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use std::sync::Arc;

pub struct ServerApp<F, D, S, P, T>
where
    F: FolderService,
    D: DocumentService,
    S: SearcherService,
    P: PaginatorService,
    T: TokenizerService,
{
    folders: Arc<F>,
    documents: Arc<D>,
    searcher: Arc<S>,
    paginator: Arc<P>,
    tokenizer: Arc<T>,
}

impl<F, D, S, P, T> ServerApp<F, D, S, P, T>
where
    F: FolderService,
    D: DocumentService,
    S: SearcherService,
    P: PaginatorService,
    T: TokenizerService,
{
    pub fn new(
        folders: Arc<F>,
        documents: Arc<D>,
        searcher: Arc<S>,
        paginator: Arc<P>,
        tokenizer: Arc<T>,
    ) -> Self {
        ServerApp {
            folders,
            documents,
            searcher,
            paginator,
            tokenizer,
        }
    }
}

pub fn init_server<F, D, S, P, T>(app: ServerApp<F, D, S, P, T>) -> Router
where
    F: FolderService + Send + Sync + 'static,
    D: DocumentService + Send + Sync + 'static,
    S: SearcherService + Send + Sync + 'static,
    P: PaginatorService + Send + Sync + 'static,
    T: TokenizerService + Send + Sync + 'static,
{
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app_arc = Arc::new(app);
    Router::new()
        .merge(swagger::init_swagger())
        .route("/storage/folders", get(router::storage::get_folders))
        .route(
            "/storage/{folder_id}",
            get(router::storage::get_folder)
                .delete(router::storage::delete_folder)
                .put(router::storage::create_folder),
        )
        .route(
            "/storage/{folder_id}/documents",
            post(router::storage::get_documents),
        )
        .route(
            "/storage/{folder_id}/{document_id}",
            post(router::storage::get_document)
                .delete(router::storage::delete_document)
                .patch(router::storage::update_document)
                .put(router::storage::create_document),
        )
        .route("/search/fulltext", post(router::searcher::search_fulltext))
        .route("/search/semantic", post(router::searcher::search_semantic))
        .route(
            "/search/paginate/next",
            post(router::searcher::paginate_next),
        )
        .route(
            "/search/paginate/sessions",
            post(router::searcher::delete_scrolls),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .with_state(app_arc)
}
