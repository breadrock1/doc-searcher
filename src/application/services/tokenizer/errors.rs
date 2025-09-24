use thiserror::Error;
use crate::application::structures::InputContentBuilderError;

pub type TokenizeResult<T> = Result<T, TokenizeError>;

#[derive(Debug, Error)]
pub enum TokenizeError {
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
    #[error("input content validation error: {0}")]
    InputFormValidation(InputContentBuilderError),
    #[error("returned empty response")]
    EmptyResponse,
}

impl From<reqwest::Error> for TokenizeError {
    fn from(err: reqwest::Error) -> Self {
        let Some(status) = err.status() else {
            return TokenizeError::ServiceError(err);
        };

        match status.as_u16() {
            503 => TokenizeError::ServiceUnavailable(err),
            408 => TokenizeError::RequestTimeout(err),
            _ => TokenizeError::ServiceError(err),
        }
    }
}
