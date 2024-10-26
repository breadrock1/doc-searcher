use crate::searcher::errors::SearcherError;
use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("request timeout: {0}")]
    RequestTimeout(String),
    #[error("returned error into response: {0}")]
    ServiceError(String),
    #[error("failed to deserialize response data: {0}")]
    SerdeError(String),
}

impl From<elasticsearch::Error> for StorageError {
    fn from(err: elasticsearch::Error) -> Self {
        let Some(status) = err.status_code() else {
            return StorageError::ServiceError(err.to_string());
        };

        match status.as_u16() {
            503 => StorageError::ServiceUnavailable(err.to_string()),
            408 => StorageError::RequestTimeout(err.to_string()),
            _ => StorageError::ServiceError(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerdeError(err.to_string())
    }
}

impl From<anyhow::Error> for StorageError {
    fn from(err: anyhow::Error) -> Self {
        StorageError::SerdeError(err.to_string())
    }
}

impl From<SearcherError> for StorageError {
    fn from(err: SearcherError) -> Self {
        StorageError::ServiceError(err.to_string())
    }
}
