use std::sync::Arc;

use mockall::mock;

use doc_search_core::shared::kernel::ResourceId;
use doc_search_core::shared::services::{IResourceManager, ResourceInfo, ResourceManagerResult};

const DEFAULT_INDEX_ID: &str = "test-index";

mock! {
    pub ResourceManagerService {}

    #[async_trait::async_trait]
    impl IResourceManager for ResourceManagerService {
        async fn get_index(&self, resource_id: &ResourceId) -> ResourceManagerResult<ResourceInfo>;
    }
}

pub struct StubResourceManager {
    index: String,
    prefix: String,
}

impl StubResourceManager {
    pub fn new(index: String, prefix: String) -> Self {
        StubResourceManager { index, prefix }
    }
}

#[async_trait::async_trait]
impl IResourceManager for StubResourceManager {
    async fn get_index(&self, _resource_id: &ResourceId) -> ResourceManagerResult<ResourceInfo> {
        Ok(ResourceInfo {
            index: self.index.clone(),
            prefix: self.prefix.clone(),
        })
    }
}

pub fn stub_resource_manager() -> Arc<dyn IResourceManager> {
    Arc::new(StubResourceManager::new(
        DEFAULT_INDEX_ID.to_string(),
        String::new(),
    ))
}
