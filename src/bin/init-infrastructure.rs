use doc_search::config::ServiceConfig;
use doc_search_core::domain::storage::models::{KnnIndexParams, KnnIndexParamsBuilder};
use doc_search_core::infrastructure::osearch::{OSearchClient, OSearchConfig};
use doc_search_core::ServiceConnect;

const APP_NAME: &str = "doc-search";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let _otlp_guard = doc_search_otlp::init_telemetry(APP_NAME, config.telemetry())?;

    let os_config = config.storage().opensearch();
    let os_client = OSearchClient::connect(os_config).await?;

    os_client.update_cluster_settings().await?;

    let knn_params = create_knn_index_params(os_config)?;
    os_client.init_pipelines(&knn_params).await?;
    os_client.load_ml_model(os_config.semantic()).await?;

    Ok(())
}

fn create_knn_index_params(config: &OSearchConfig) -> anyhow::Result<KnnIndexParams> {
    let knn_config = config.semantic();
    let knn_params = KnnIndexParamsBuilder::default()
        .knn_dimension(knn_config.knn_dimension())
        .token_limit(knn_config.token_limit())
        .overlap_rate(knn_config.overlap_rate())
        .build()?;

    Ok(knn_params)
}
