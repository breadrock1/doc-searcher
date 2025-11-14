use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("storage: auth failed: {0}")]
    AuthenticationFailed(anyhow::Error),
    #[error("storage: index has not been founded: {0}")]
    IndexNotFound(anyhow::Error),
    #[error("storage: document already exists: {0}")]
    DocumentAlreadyExists(anyhow::Error),
    #[error("storage: document has not been founded: {0}")]
    DocumentNotFound(anyhow::Error),
    #[error("storage: validation error: {0}")]
    ValidationError(anyhow::Error),
    #[error("storage: returned error into response: {0}")]
    ServiceError(anyhow::Error),
    #[error("storage: internal error: {0}")]
    InternalError(anyhow::Error),
    #[error("storage: failed to deserialize response data: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("storage: http request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),
}
