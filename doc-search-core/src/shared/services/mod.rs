#[cfg(test)]
mod tests;

mod error;
pub use error::ResourceManagerError;
pub use error::ResourceManagerResult;

mod model;
use model::ResourceInfo;

use crate::shared::kernel::ResourceId;

/// Domain service for resolving resources to search indexes.
#[async_trait::async_trait]
pub trait IResourceManager: Send + Sync {
    async fn get_index(&self, resource_id: &ResourceId) -> ResourceManagerResult<ResourceInfo>;
}
