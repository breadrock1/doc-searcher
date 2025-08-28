#![allow(dead_code)]

use anyhow::Error;
use axum::http::StatusCode;
use serde_derive::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::application::services::storage::error::StorageError;

const UNAVAILABLE_SERVER: &str = "server unavailable";

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error, Serialize, ToSchema)]
pub enum ServerError {
    #[error("server: auth failed: {0}")]
    AuthenticationFailed(String),
    #[error("server: resource data conflict: {0}")]
    Conflict(String),
    #[error("server: not found error: {0}")]
    NotFound(String),
    #[error("server: internal service error: {0}")]
    InternalError(String),
    #[error("server: bad request: {0}")]
    BadRequest(String),
    #[error("server: incorrect input form: {0}")]
    IncorrectInputForm(String),
    #[error("server: server unavailable")]
    ServerUnavailable,
}

impl From<StorageError> for ServerError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::AuthenticationFailed(err) => ServerError::AuthenticationFailed(err.to_string()),
            StorageError::IndexNotFound(err) => ServerError::NotFound(err.to_string()),
            StorageError::DocumentAlreadyExists(err) => ServerError::Conflict(err.to_string().to_string()),
            StorageError::DocumentNotFound(err) => ServerError::NotFound(err.to_string()),
            StorageError::ServiceError(err) => ServerError::BadRequest(err.to_string()),
            StorageError::InternalError(err) => ServerError::InternalError(err.to_string()),
            StorageError::ValidationError(err) => ServerError::IncorrectInputForm(err.to_string()),
            StorageError::SerdeError(err) => ServerError::InternalError(err.to_string()),
            StorageError::HttpRequestError(err) => ServerError::InternalError(err.to_string()),
        }
    }
}

impl From<anyhow::Error> for ServerError {
    fn from(err: Error) -> Self {
        ServerError::IncorrectInputForm(err.to_string())
    }
}

impl ServerError {
    pub fn status_code(&self) -> (StatusCode, &str) {
        match self {
            ServerError::AuthenticationFailed(err) => (StatusCode::UNAUTHORIZED, err),
            ServerError::NotFound(err) => (StatusCode::NOT_FOUND, err),
            ServerError::Conflict(err) => (StatusCode::CONFLICT, err),
            ServerError::InternalError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            ServerError::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
            ServerError::IncorrectInputForm(err) => (StatusCode::BAD_REQUEST, err),
            ServerError::ServerUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                UNAVAILABLE_SERVER,
            ),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct Success {
    #[schema(example = 200)]
    status: u16,
    #[schema(example = "Ok")]
    message: String,
}

impl Default for Success {
    fn default() -> Self {
        let status_code = StatusCode::OK;
        Success {
            status: status_code.as_u16(),
            message: status_code.to_string(),
        }
    }
}

impl Success {
    pub fn new(status: u16, message: &str) -> Self {
        let message = message.to_string();
        Success {
            status,
            message,
        }
    }
}
