use doc_search::application::structures::params::{KnnIndexParams, KnnIndexParamsBuilder};
use doc_search::config::ServiceConfig;
use doc_search::infrastructure::osearch::config::OSearchConfig;
use doc_search::infrastructure::osearch::OpenSearchStorage;
use doc_search::{telemetry, ServiceConnect};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let _ = telemetry::init_otlp_tracing(config.otlp())?;

    let os_config = config.storage().opensearch();
    let os_client = OpenSearchStorage::connect(os_config).await?;

    let knn_params = create_knn_index_params(os_config)?;
    os_client.init_pipelines(&knn_params).await?;

    Ok(())
}

fn create_knn_index_params(config: &OSearchConfig) -> anyhow::Result<KnnIndexParams> {
    let knn_params = KnnIndexParams::default();

    let knn_config = config.semantic();
    let knn_ef_searcher = knn_config
        .knn_ef_searcher()
        .unwrap_or(knn_params.knn_ef_searcher());
    let knn_dimension = knn_config
        .knn_dimension()
        .unwrap_or(knn_params.knn_dimension());
    let token_limit = knn_config.token_limit().unwrap_or(knn_params.token_limit());
    let overlap_rate = knn_config
        .overlap_rate()
        .unwrap_or(knn_params.overlap_rate());

    let knn_params = KnnIndexParamsBuilder::default()
        .knn_ef_searcher(knn_ef_searcher)
        .knn_dimension(knn_dimension)
        .token_limit(token_limit)
        .overlap_rate(overlap_rate)
        .build()?;

    Ok(knn_params)
}
