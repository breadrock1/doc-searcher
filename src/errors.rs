use crate::searcher::models::Paginated;

use crate::embeddings::errors::EmbeddingsError;
use crate::searcher::errors::SearcherError;
use crate::storage::errors::StorageError;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

pub type WebResult<T> = Result<T, WebError>;
pub type JsonResponse<T> = Result<web::Json<T>, WebError>;
pub type PaginateResponse<T> = JsonResponse<Paginated<T>>;

#[derive(Debug, Error)]
pub enum WebError {
    #[error("cache component error: {0}")]
    CacherError(String),
    #[error("metrics component error: {0}")]
    MetricsError(String),
    #[error("embeddings component error: {0}")]
    EmbeddingsError(String),
    #[error("storage component error: {0}")]
    StorageError(String),
    #[error("failed while searching: {0}")]
    SearchingError(String),
    #[error("failed to (de)serialize object: {0}")]
    SerdeError(String),

    #[error("continues executing: {0}")]
    Continues(String),
    #[error("service unavailable: {0}")]
    Unavailable(String),
    #[error("unexpected runtime error: {0}")]
    RuntimeError(String),
}

impl WebError {
    pub fn name(&self) -> &str {
        match self {
            WebError::CacherError(_) => "Cache error",
            WebError::MetricsError(_) => "Metrics error",
            WebError::EmbeddingsError(_) => "Embeddings error",
            WebError::StorageError(_) => "Storage error",
            WebError::SearchingError(_) => "Searching error",
            WebError::SerdeError(_) => "Serialize/Deserialize error",

            WebError::Continues(_) => "Processing...",
            WebError::Unavailable(_) => "Service unavailable",
            WebError::RuntimeError(_) => "Unexpected runtime error",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ErrorResponse {
    pub fn new(code: u16, err: &str, msg: &str) -> Self {
        ErrorResponse {
            code,
            error: err.to_string(),
            message: msg.to_string(),
        }
    }
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::CacherError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::MetricsError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::EmbeddingsError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::StorageError(_) => StatusCode::BAD_REQUEST,
            WebError::SearchingError(_) => StatusCode::BAD_REQUEST,
            WebError::SerdeError(_) => StatusCode::BAD_REQUEST,

            WebError::Continues(_) => StatusCode::PROCESSING,
            WebError::Unavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            WebError::RuntimeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name().to_string(),
        };

        HttpResponse::build(status_code).json(response)
    }
}

impl From<elasticsearch::http::response::Exception> for WebError {
    fn from(ex: elasticsearch::http::response::Exception) -> Self {
        let err = ex.error();
        tracing::error!("elasticsearch exception: {err:#?}");

        let msg = err.reason().unwrap_or_default();
        WebError::RuntimeError(msg.to_string())
    }
}

impl From<elasticsearch::Error> for WebError {
    fn from(err: elasticsearch::Error) -> Self {
        tracing::error!("elasticsearch error: {err:#?}");
        WebError::SearchingError(err.to_string())
    }
}

impl From<serde_json::Error> for WebError {
    fn from(err: serde_json::Error) -> Self {
        tracing::error!("serde error: {err:#?}");
        WebError::SerdeError(err.to_string())
    }
}

impl From<reqwest::Error> for WebError {
    fn from(err: reqwest::Error) -> Self {
        tracing::error!("reqwest error: {err:#?}");
        WebError::RuntimeError(err.to_string())
    }
}

impl From<EmbeddingsError> for WebError {
    fn from(err: EmbeddingsError) -> Self {
        WebError::EmbeddingsError(err.to_string())
    }
}

impl From<StorageError> for WebError {
    fn from(err: StorageError) -> Self {
        WebError::StorageError(err.to_string())
    }
}

impl From<SearcherError> for WebError {
    fn from(err: SearcherError) -> Self {
        WebError::SearchingError(err.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Getters, CopyGetters, ToSchema)]
pub struct Successful {
    #[getset(get_copy = "pub")]
    code: u16,
    #[getset(get = "pub")]
    message: String,
}

impl Default for Successful {
    fn default() -> Self {
        Successful::new(200, "Done")
    }
}

impl Successful {
    pub fn new(code: u16, msg: &str) -> Self {
        let message = msg.to_string();
        Successful { code, message }
    }
}
