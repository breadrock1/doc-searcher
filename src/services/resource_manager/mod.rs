mod config;
pub use config::ResourceManagerConfig;

mod dto;
pub use dto::ResourceSchema;

use doc_search_core::ServiceConnect;
use doc_search_core::shared::kernel::ResourceId;
use doc_search_core::shared::services::{IResourceManager, ResourceInfo, ResourceManagerError, ResourceManagerResult};
use metrics::{counter, histogram};
use std::time::Duration;
use tokio::time::Instant;
use tracing::instrument;

const RESOURCE_PATH: &str = "/api/v1/resource";

#[derive(Clone)]
pub struct ResourceManagerClient {
    config: ResourceManagerConfig,
    client: reqwest::Client,
}

#[async_trait::async_trait]
impl ServiceConnect for ResourceManagerClient {
    type Config = ResourceManagerConfig;
    type Client = ResourceManagerClient;
    type Error = reqwest::Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout()))
            .build()?;

        tracing::debug!(
            base_url = %config.base_url(),
            "connected to resource-manager service"
        );

        Ok(ResourceManagerClient {
            config: config.clone(),
            client,
        })
    }
}

#[async_trait::async_trait]
impl IResourceManager for ResourceManagerClient {
    #[instrument(level = "info", skip(self), fields(resource_id = %resource_id))]
    async fn get_index(&self, resource_id: &ResourceId) -> ResourceManagerResult<ResourceInfo> {
        let target_url = format!(
            "{}{RESOURCE_PATH}/{}",
            self.config.base_url().trim_end_matches('/'),
            resource_id,
        );

        let instant = Instant::now();
        let response = self.client.get(target_url).send().await;
        let status = match &response {
            Ok(response) => response.status().as_u16().to_string(),
            Err(_) => "error".to_string(),
        };

        counter!(
            "docsearch_resource_manager_requests_total",
            "status" => status,
        )
            .increment(1);
        histogram!("docsearch_resource_manager_request_duration_seconds")
            .record(instant.elapsed().as_secs_f64());

        let response =
            response.map_err(|err| ResourceManagerError::InternalError(err.to_string()))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ResourceManagerError::NotFound(resource_id.to_string()));
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();

            return Err(ResourceManagerError::InternalError(format!(
                "unexpected status {status}: {body}"
            )));
        }

        let schema = response.json::<ResourceSchema>().await.map_err(|err| {
            ResourceManagerError::InternalError(format!("deserialize error: {err}"))
        })?;

        let index = schema
            .external_id
            .filter(|it| !it.trim().is_empty())
            .ok_or_else(|| {
                ResourceManagerError::InternalError("external_id is empty".to_string())
            })?;

        Ok(ResourceInfo {
            index,
            prefix: schema.prefix,
        })
    }
}
