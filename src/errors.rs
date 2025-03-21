use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::engine::error::{SearcherError, StorageError};
use crate::engine::model::Paginated;
use crate::tokenizer::errors::TokenizerError;

pub type ServerResult<T> = Result<T, ServerError>;
pub type PaginateResponse<T> = ServerResult<Paginated<T>>;

#[derive(Debug, Error)]
pub enum ServerError {
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

impl ServerError {
    pub fn status_code(&self) -> (String, StatusCode) {
        match self {
            ServerError::CacherError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::MetricsError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::EmbeddingsError(msg) => {
                (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR)
            }
            ServerError::StorageError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::SearchingError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::SerdeError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::Continues(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::Unavailable(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::RuntimeError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
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

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (message, status) = self.status_code();
        let mut resp = Json(ErrorResponse { message }).into_response();

        *resp.status_mut() = status;
        resp
    }
}

impl From<elasticsearch::http::response::Exception> for ServerError {
    fn from(ex: elasticsearch::http::response::Exception) -> Self {
        let err = ex.error();
        tracing::error!("elasticsearch exception: {err:#?}");

        let msg = err.reason().unwrap_or_default();
        ServerError::RuntimeError(msg.to_string())
    }
}

impl From<elasticsearch::Error> for ServerError {
    fn from(err: elasticsearch::Error) -> Self {
        tracing::error!("elasticsearch error: {err:#?}");
        ServerError::SearchingError(err.to_string())
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(err: serde_json::Error) -> Self {
        tracing::error!("serde error: {err:#?}");
        ServerError::SerdeError(err.to_string())
    }
}

impl From<reqwest::Error> for ServerError {
    fn from(err: reqwest::Error) -> Self {
        tracing::error!("request error: {err:#?}");
        ServerError::RuntimeError(err.to_string())
    }
}

impl From<TokenizerError> for ServerError {
    fn from(err: TokenizerError) -> Self {
        ServerError::EmbeddingsError(err.to_string())
    }
}

impl From<StorageError> for ServerError {
    fn from(err: StorageError) -> Self {
        ServerError::StorageError(err.to_string())
    }
}

impl From<SearcherError> for ServerError {
    fn from(err: SearcherError) -> Self {
        ServerError::SearchingError(err.to_string())
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
