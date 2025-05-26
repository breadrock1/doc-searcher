use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use doc_search::config::ServiceConfig;
use doc_search::engine::elastic::ElasticClient;
use doc_search::server::ServerApp;
use doc_search::tokenizer::baai::BAAIClient;
use doc_search::{logger, server, ServiceConnect};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{cors, trace};

#[tokio::main(worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    logger::init_logger(config.logger())?;

    let tokenizer = Arc::new(BAAIClient::connect(config.tokenizer().baai()).await?);
    let searcher = Arc::new(ElasticClient::connect(config.elastic()).await?);
    let server_app = ServerApp::new(
        searcher.clone(),
        searcher.clone(),
        searcher.clone(),
        searcher.clone(),
        tokenizer,
    );

    let cors_layer = cors::CorsLayer::permissive();
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let app = server::init_server(server_app)
        .layer(trace_layer)
        .layer(cors_layer)
        .layer(OtelAxumLayer::default());

    let server_config = config.server();
    let listener = TcpListener::bind(server_config.address()).await?;
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(err=?err, "failed to stop http server");
    };

    Ok(())
}
