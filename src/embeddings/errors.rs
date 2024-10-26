use thiserror::Error;

pub type EmbeddingsResult<T> = Result<T, EmbeddingsError>;

#[derive(Debug, Error)]
pub enum EmbeddingsError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("request timeout: {0}")]
    RequestTimeout(String),
    #[error("returned error into response: {0}")]
    ServiceError(String),
    #[error("failed to deserialize response data: {0}")]
    SerdeError(String),
}

impl From<reqwest::Error> for EmbeddingsError {
    fn from(err: reqwest::Error) -> Self {
        let Some(status) = err.status() else {
            return EmbeddingsError::ServiceError(err.to_string());
        };

        match status.as_u16() {
            503 => EmbeddingsError::ServiceUnavailable(err.to_string()),
            408 => EmbeddingsError::RequestTimeout(err.to_string()),
            _ => EmbeddingsError::ServiceError(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for EmbeddingsError {
    fn from(err: serde_json::Error) -> Self {
        EmbeddingsError::SerdeError(err.to_string())
    }
}
