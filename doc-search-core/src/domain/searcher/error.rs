use thiserror::Error;

pub type SearchResult<T> = Result<T, SearchError>;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("searcher: auth failed: {0}")]
    AuthenticationFailed(anyhow::Error),
    #[error("searcher: auth failed: {0}")]
    ConnectionError(anyhow::Error),
    #[error("searcher: index has not been founded: {0}")]
    IndexNotFound(anyhow::Error),
    #[error("searcher: validation error: {0}")]
    ValidationError(anyhow::Error),
    #[error("searcher: returned error into response: {0}")]
    ServiceError(anyhow::Error),
    #[error("searcher: internal error: {0}")]
    InternalError(anyhow::Error),
    #[error("searcher: unknown error: {0}")]
    UnknownError(anyhow::Error),
}
