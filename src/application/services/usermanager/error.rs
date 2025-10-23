use thiserror::Error;

pub type UserManagerResult<T> = Result<T, UserManagerError>;

#[derive(Debug, Error)]
pub enum UserManagerError {
    #[error("storage: auth failed: {0}")]
    AuthenticationFailed(anyhow::Error),
    #[error("storage: index has not been founded: {0}")]
    ResourceNotFound(anyhow::Error),
    #[error("storage: returned error into response: {0}")]
    ServiceError(anyhow::Error),
    #[error("storage: internal error: {0}")]
    InternalError(anyhow::Error),
    #[error("storage: failed to deserialize response data: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("storage: http request failed: {0}")]
    HttpRequestError(#[from] reqwest::Error),
}
