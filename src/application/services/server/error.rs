#![allow(dead_code)]

use axum::http::StatusCode;
use serde_derive::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::application::services::storage::error::StorageError;

const UNAVAILABLE_SERVER: &str = "server unavailable";

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error, Serialize, ToSchema)]
pub enum ServerError {
    #[error("not found error: {0}")]
    NotFound(String),
    #[error("internal service error: {0}")]
    InternalError(String),
    #[error("server unavailable")]
    ServerUnavailable,
}

impl From<StorageError> for ServerError {
    fn from(err: StorageError) -> Self {
        ServerError::InternalError(err.to_string())
    }
}

impl ServerError {
    pub fn status_code(&self) -> (StatusCode, String) {
        match self {
            ServerError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.to_owned()),
            ServerError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_owned()),
            ServerError::ServerUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                UNAVAILABLE_SERVER.to_owned(),
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
        Success {
            status: 200,
            message: "Ok".to_string(),
        }
    }
}

impl Success {
    pub fn new(status: u16, message: &str) -> Self {
        Success {
            status,
            message: message.to_owned(),
        }
    }
}
