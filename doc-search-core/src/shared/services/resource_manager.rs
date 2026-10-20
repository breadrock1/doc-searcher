use std::fmt::Display;

use thiserror::Error;

/// A resource identifier.
///
/// Represents the unique identifier of a resource (e.g. a tenant or service)
/// managed by the resource manager.
#[derive(Clone, Debug)]
pub struct ResourceId(pub String);

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for ResourceId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ResourceId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

/// Information about a resource location.
///
/// Describes the search index and prefix where a resource's data is stored.
#[derive(Clone, Debug)]
pub struct ResourceInfo {
    pub index: String,
    pub prefix: String,
}

/// Represents possible errors that can occur during resource manager operations.
#[derive(Debug, Error)]
pub enum ResourceManagerError {
    /// The requested resource does not exist.
    #[error("resource not found: {0}")]
    NotFound(String),

    /// Internal resource manager error.
    #[error("resource manager error: {0}")]
    InternalError(String),
}

/// Type alias for resource manager operation results.
pub type ResourceManagerResult<T> = Result<T, ResourceManagerError>;

/// Domain service for resolving resources to search indexes.
#[async_trait::async_trait]
pub trait IResourceManager: Send + Sync {
    /// Resolves a resource id to the index and prefix where its data is stored.
    async fn get_index(&self, resource_id: &ResourceId) -> ResourceManagerResult<ResourceInfo>;
}
