use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("storage: auth failed: {0}")]
    AuthenticationFailed(anyhow::Error),
    #[error("storage: connection error: {0}")]
    ConnectionError(anyhow::Error),
    #[error("storage: index has not been founded: {0}")]
    IndexNotFound(anyhow::Error),
    #[error("storage: document has not been founded: {0}")]
    DocumentNotFound(anyhow::Error),
    #[error("storage: document already exists: {0}")]
    DocumentAlreadyExists(anyhow::Error),
    #[error("can't split large document: {0}")]
    CantSplitLargeDocuments(anyhow::Error),
    #[error("storage: validation error: {0}")]
    ValidationError(anyhow::Error),
    #[error("storage: internal error: {0}")]
    InternalError(anyhow::Error),
    #[error("storage: unknown error: {0}")]
    UnknownError(anyhow::Error),
}
