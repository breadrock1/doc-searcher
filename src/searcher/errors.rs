use crate::searcher::models::Paginated;
use thiserror::Error;

pub type SearcherResult<T> = Result<T, SearcherError>;
pub type PaginatedResult<T> = Result<Paginated<Vec<T>>, SearcherError>;

#[derive(Debug, Error)]
pub enum SearcherError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("failed to paginate: {0}")]
    PaginateError(String),
    #[error("request timeout: {0}")]
    RequestTimeout(String),
    #[error("returned error into response: {0}")]
    ServiceError(String),
    #[error("failed to deserialize response data: {0}")]
    SerdeError(String),
}

impl From<elasticsearch::Error> for SearcherError {
    fn from(err: elasticsearch::Error) -> Self {
        let Some(status) = err.status_code() else {
            return SearcherError::ServiceError(err.to_string());
        };

        match status.as_u16() {
            503 => SearcherError::ServiceUnavailable(err.to_string()),
            408 => SearcherError::RequestTimeout(err.to_string()),
            _ => SearcherError::ServiceError(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for SearcherError {
    fn from(err: serde_json::Error) -> Self {
        SearcherError::SerdeError(err.to_string())
    }
}
