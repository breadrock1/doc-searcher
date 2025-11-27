use axum::Router;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use doc_search::config::ServiceConfig;
use doc_search::server::{httpserver, ServerApp};
use doc_search_core::application::usecase::searcher::SearcherUseCase;
use doc_search_core::application::usecase::storage::StorageUseCase;
use doc_search_core::infrastructure::osearch::OSearchClient;
use doc_search_core::ServiceConnect;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower_http::{cors, trace};

const APP_NAME: &str = "doc-search";

#[tokio::main(worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let _otlp_guard = doc_search_otlp::init_otlp_tracing(APP_NAME, config.otlp())?;

    let osearch_config = config.storage().opensearch();
    let osearch_client = Arc::new(OSearchClient::connect(osearch_config).await?);

    let max_content_size = config.settings().max_content_size();
    let storage_uc = StorageUseCase::new(osearch_client.clone(), max_content_size);
    let searcher_uc = SearcherUseCase::new(osearch_client.clone());
    let server_app = ServerApp::new(storage_uc, searcher_uc);

    let cors_layer = cors::CorsLayer::permissive();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(doc_search_otlp::PathFilter::default())
        .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::ERROR));

    let app = httpserver::init_server(server_app)
        .layer(trace_layer)
        .layer(cors_layer)
        .layer(OtelAxumLayer::default());

    let cache_config = config.cache();
    let tmp_app_state: anyhow::Result<Router> = match cache_config.is_enabled() {
        false => Ok(app),
        true => {
            let redis_config = cache_config.redis();
            let app = httpserver::mw::cache::enable_caching_mw(app, redis_config).await?;
            Ok(app)
        }
    };

    let app = tmp_app_state?;
    let server_config = config.server();
    let listener = TcpListener::bind(server_config.http().address()).await?;
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(err=?err, "failed to stop http server");
    };

    Ok(())
}
