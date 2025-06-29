use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use doc_search::application::services::server::ServerApp;
use doc_search::application::services::tokenizer::Tokenizer;
use doc_search::application::{SearcherUseCase, StorageUseCase};
use doc_search::config::ServiceConfig;
use doc_search::infrastructure::osearch::OpenSearchStorage;
use doc_search::infrastructure::httpserver;
use doc_search::infrastructure::vectorizer::VectorizerClient;
use doc_search::{logger, ServiceConnect};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{cors, trace};

#[tokio::main(worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    logger::init_logger(config.logger())?;

    let tokenizer: Option<Arc<Box<dyn Tokenizer + Send + Sync>>> = match config.tokenizer() {
        None => None,
        Some(tokenizer_config) => {
            let baii_config = tokenizer_config.baai();
            let baii_client = VectorizerClient::connect(baii_config).await?;
            Some(Arc::new(Box::new(baii_client)))
        }
    };

    let osearch_config = config.storage().opensearch();
    let osearch_client = Arc::new(OpenSearchStorage::connect(osearch_config).await?);

    let storage_uc = StorageUseCase::new(osearch_client.clone());
    let searcher_uc = SearcherUseCase::new(osearch_client.clone(), tokenizer);
    let server_app = ServerApp::new(storage_uc, searcher_uc);

    let cors_layer = cors::CorsLayer::permissive();
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let app = httpserver::init_server(server_app)
        .layer(trace_layer)
        .layer(cors_layer)
        .layer(OtelAxumLayer::default());

    #[cfg(feature = "enable-cache-redis")]
    let app = httpserver::mw::cache::enable_caching_mw(app, config.cacher().redis()).await?;

    let server_config = config.server();
    let listener = TcpListener::bind(server_config.address()).await?;
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(err=?err, "failed to stop http server");
    };

    Ok(())
}
