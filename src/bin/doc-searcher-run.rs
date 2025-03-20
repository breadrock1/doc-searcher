extern crate doc_search;

use doc_search::config;
use doc_search::engine::elastic::ElasticClient;
use doc_search::server::ServerApp;
use doc_search::tokenizer::baai::BAAIClient;
use doc_search::{logger, server, ServiceConnect};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{cors, trace};

#[tokio::main(worker_threads = 6)]
async fn main() -> anyhow::Result<()> {
    let s_config = config::ServiceConfig::new()?;

    let logger_config = s_config.logger();
    logger::init_logger(logger_config)?;

    let tokenizer = Arc::new(BAAIClient::connect(s_config.tokenizer().baai()).await?);
    let searcher = Arc::new(ElasticClient::connect(s_config.elastic()).await?);

    let server_config = s_config.server();
    let listener = TcpListener::bind(server_config.address()).await?;
    let server_app = ServerApp::new(
        searcher.clone(),
        searcher.clone(),
        searcher.clone(),
        searcher.clone(),
        tokenizer,
    );

    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let cors_layer = cors::CorsLayer::permissive();

    let app = server::init_server(server_app)
        .layer(trace_layer)
        .layer(cors_layer);

    axum::serve(listener, app).await?;

    Ok(())
}
