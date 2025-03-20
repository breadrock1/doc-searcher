use thiserror::Error;

use crate::engine::model::Paginated;

pub type StorageResult<T> = Result<T, StorageError>;
pub type SearcherResult<T> = Result<T, SearcherError>;
pub type PaginatedResult<T> = Result<Paginated<Vec<T>>, SearcherError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("request timeout: {0}")]
    RequestTimeout(String),
    #[error("target object haven't been founded: {0}")]
    NotFound(String),
    #[error("returned error into response: {0}")]
    ServiceError(String),
    #[error("failed to deserialize response data: {0}")]
    SerdeError(String),
}

#[derive(Debug, Error)]
pub enum SearcherError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(elasticsearch::Error),
    #[error("failed to paginate: {0}")]
    PaginateError(String),
    #[error("request timeout: {0}")]
    RequestTimeout(elasticsearch::Error),
    #[error("returned error into response: {0}")]
    ServiceError(elasticsearch::Error),
    #[error("returned error into response: {0}")]
    RuntimeError(String),
    #[error("failed to deserialize response data: {0}")]
    SerdeError(#[from] serde_json::Error),
}

impl From<elasticsearch::Error> for SearcherError {
    fn from(err: elasticsearch::Error) -> Self {
        let Some(status) = err.status_code() else {
            return SearcherError::ServiceError(err);
        };

        match status.as_u16() {
            503 => SearcherError::ServiceUnavailable(err),
            408 => SearcherError::RequestTimeout(err),
            _ => SearcherError::ServiceError(err),
        }
    }
}

impl From<elasticsearch::Error> for StorageError {
    fn from(err: elasticsearch::Error) -> Self {
        let Some(status) = err.status_code() else {
            return StorageError::ServiceError(err.to_string());
        };

        match status.as_u16() {
            503 => StorageError::ServiceUnavailable(err.to_string()),
            404 => StorageError::NotFound(err.to_string()),
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

impl From<SearcherError> for StorageError {
    fn from(err: SearcherError) -> Self {
        StorageError::ServiceError(err.to_string())
    }
}
