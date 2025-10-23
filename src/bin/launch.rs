use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use doc_search::application::services::server::ServerApp;
use doc_search::application::services::usermanager::UserManager;
use doc_search::application::{SearcherUseCase, StorageUseCase};
use doc_search::config::ServiceConfig;
use doc_search::infrastructure::httpserver;
use doc_search::infrastructure::osearch::OpenSearchStorage;
use doc_search::infrastructure::usermanager::UserManagerClient;
use doc_search::{telemetry, ServiceConnect};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower_http::{cors, trace};

#[tokio::main(worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let _otlp_guard = telemetry::init_otlp_tracing(config.otlp())?;

    let osearch_config = config.storage().opensearch();
    let osearch_client = Arc::new(OpenSearchStorage::connect(osearch_config).await?);

    let um_config = config.server().usermanager();
    let um: Arc<Box<dyn UserManager  + Send + Sync + 'static>> = Arc::new(Box::new(UserManagerClient::connect(um_config).await?));

    let storage_uc = StorageUseCase::new(config.settings(), osearch_client.clone(), um.clone());
    let searcher_uc = SearcherUseCase::new(osearch_client.clone());
    let server_app = ServerApp::new(storage_uc, searcher_uc);

    let cors_layer = cors::CorsLayer::permissive();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(telemetry::PathFilter::default())
        .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::ERROR));

    let app = httpserver::init_server(server_app)
        .layer(trace_layer)
        .layer(cors_layer)
        .layer(OtelAxumLayer::default());

    let app = httpserver::mw::header::enable_header_extractor_mw(app).await?;

    #[cfg(feature = "enable-cache-redis")]
    let app = httpserver::mw::cache::enable_caching_mw(app, config.cacher().redis()).await?;

    let server_config = config.server();
    let listener = TcpListener::bind(server_config.http().address()).await?;
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(err=?err, "failed to stop http server");
    };

    Ok(())
}
