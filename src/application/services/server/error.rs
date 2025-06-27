#![allow(dead_code)]

use axum::http::StatusCode;
use serde_derive::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

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

#[derive(Serialize, ToSchema)]
pub struct Success {
    status: u16,
    message: String,
}

impl Default for Success {
    fn default() -> Self {
        Success {
            status: 200,
            message: "Ok".to_string(),
        }
    }
}
