use doc_search::config::ServiceConfig;
use doc_search_core::infrastructure::osearch::OSearchClient;
use doc_search_core::ServiceConnect;

const SERVICE_NAME: &str = "doc-search-migrator";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let _otlp_guard = otlp::init_telemetry(SERVICE_NAME, config.telemetry())?;

    let os_config = config.storage().opensearch();
    let os_client = OSearchClient::connect(os_config).await?;

    os_client.init_pipelines().await?;

    Ok(())
}
