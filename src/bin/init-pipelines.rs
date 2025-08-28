use doc_search::application::structures::params::KnnIndexParams;
use doc_search::config::ServiceConfig;
use doc_search::infrastructure::osearch::OpenSearchStorage;
use doc_search::{tracer, ServiceConnect};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    tracer::init_otlp_tracing(&config)?;

    let os_config = config.storage().opensearch();
    let os_client = OpenSearchStorage::connect(os_config).await?;

    let knn_params = KnnIndexParams::default();
    os_client.init_pipelines(&knn_params).await?;

    Ok(())
}
