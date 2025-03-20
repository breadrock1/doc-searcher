use thiserror::Error;

use crate::engine::model::Paginated;

pub type StorageResult<T> = Result<T, StorageError>;
pub type SearcherResult<T> = Result<T, SearcherError>;
pub type PaginatedResult<T> = Result<Paginated<Vec<T>>, SearcherError>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(elasticsearch::Error),
    #[error("request timeout: {0}")]
    RequestTimeout(elasticsearch::Error),
    #[error("target object haven't been founded: {0}")]
    NotFound(elasticsearch::Error),
    #[error("returned error into response: {0}")]
    ServiceError(elasticsearch::Error),
    #[error("failed to deserialize response data: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("returned error into response: {0}")]
    RuntimeError(String),
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
    #[error("resource not found: {0}")]
    NotFound(elasticsearch::Error),
}

impl From<elasticsearch::Error> for SearcherError {
    fn from(err: elasticsearch::Error) -> Self {
        let Some(status) = err.status_code() else {
            return SearcherError::ServiceError(err);
        };

        match status.as_u16() {
            503 => SearcherError::ServiceUnavailable(err),
            408 => SearcherError::RequestTimeout(err),
            404 => SearcherError::NotFound(err),
            _ => SearcherError::ServiceError(err),
        }
    }
}

impl From<elasticsearch::Error> for StorageError {
    fn from(err: elasticsearch::Error) -> Self {
        let Some(status) = err.status_code() else {
            return StorageError::ServiceError(err);
        };

        match status.as_u16() {
            503 => StorageError::ServiceUnavailable(err),
            404 => StorageError::NotFound(err),
            408 => StorageError::RequestTimeout(err),
            _ => StorageError::ServiceError(err),
        }
    }
}

impl From<SearcherError> for StorageError {
    fn from(err: SearcherError) -> Self {
        match err {
            SearcherError::ServiceUnavailable(err) => StorageError::ServiceUnavailable(err),
            SearcherError::RequestTimeout(err) => StorageError::RequestTimeout(err),
            SearcherError::ServiceError(err) => StorageError::ServiceError(err),
            SearcherError::SerdeError(err) => StorageError::SerdeError(err),
            SearcherError::NotFound(err) => StorageError::NotFound(err),
            SearcherError::RuntimeError(msg) | SearcherError::PaginateError(msg) => {
                StorageError::RuntimeError(msg)
            }
        }
    }
}
