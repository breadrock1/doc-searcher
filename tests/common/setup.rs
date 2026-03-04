use doc_search::config::ServiceConfig;
use doc_search_core::domain::storage::models::CreateIndexParamsBuilder;
use doc_search_core::domain::storage::IIndexStorage;
use doc_search_core::infrastructure::osearch::OSearchClient;
use doc_search_core::ServiceConnect;
use std::sync::Arc;

#[derive(Clone)]
pub struct TestEnvironment {
    index: String,
    osearch: Arc<OSearchClient>,
}

impl TestEnvironment {
    pub fn osearch(&self) -> Arc<OSearchClient> {
        self.osearch.clone()
    }

    pub fn get_index(&self) -> &String {
        &self.index
    }

    pub async fn teardown(&self) -> anyhow::Result<()> {
        // TODO: This operation does not need
        // self.osearch().delete_index(index_id).await?;
        Ok(())
    }
}

pub async fn setup_osearch_environment(index_id: &str) -> anyhow::Result<TestEnvironment> {
    let config = ServiceConfig::new()?;
    let config = config.storage().opensearch();
    let client = OSearchClient::connect(config).await?;

    let _ = client.delete_index(index_id).await;
    let create_index_params = CreateIndexParamsBuilder::default()
        .id(index_id.to_string())
        .knn(None)
        .build()?;

    let _ = client.create_index(&create_index_params).await?;

    Ok(TestEnvironment {
        index: index_id.to_string(),
        osearch: Arc::new(client),
    })
}
