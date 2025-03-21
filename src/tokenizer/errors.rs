use thiserror::Error;

pub type TokenizerResult<T> = Result<T, TokenizerError>;

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(reqwest::Error),
    #[error("request timeout: {0}")]
    RequestTimeout(reqwest::Error),
    #[error("returned error into response: {0}")]
    ServiceError(reqwest::Error),
    #[error("failed to deserialize response data: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("failed to connect: {0}")]
    ConnectError(String),
    #[error("returned empty response")]
    EmptyResponse,
}

impl From<reqwest::Error> for TokenizerError {
    fn from(err: reqwest::Error) -> Self {
        let Some(status) = err.status() else {
            return TokenizerError::ServiceError(err);
        };

        match status.as_u16() {
            503 => TokenizerError::ServiceUnavailable(err),
            408 => TokenizerError::RequestTimeout(err),
            _ => TokenizerError::ServiceError(err),
        }
    }
}
