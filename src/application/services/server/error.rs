#![allow(dead_code)]

use axum::http::StatusCode;
use serde_derive::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::application::services::storage::error::StorageError;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("not found error: {0}")]
    NotFound(String),
    #[error("internal service error: {0}")]
    InternalError(String),
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl From<StorageError> for ServerError {
    fn from(err: StorageError) -> Self {
        ServerError::InternalError(err.to_string())
    }
}

impl ServerError {
    pub fn status_code(&self) -> (String, StatusCode) {
        match self {
            ServerError::NotFound(msg) => (msg.to_owned(), StatusCode::NOT_FOUND),
            ServerError::InternalError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::ServiceUnavailable(msg) => {
                (msg.to_owned(), StatusCode::SERVICE_UNAVAILABLE)
            }
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

impl Success {
    fn new(status: u16, message: &str) -> Self {
        Success {
            status,
            message: message.to_owned(),
        }
    }
}
