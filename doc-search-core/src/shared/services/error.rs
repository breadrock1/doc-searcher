use thiserror::Error;

/// Type alias for resource manager operation results.
pub type ResourceManagerResult<T> = Result<T, ResourceManagerError>;

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
