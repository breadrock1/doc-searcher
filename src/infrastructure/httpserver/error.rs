use crate::application::services::storage::error::StorageError;
use crate::infrastructure::httpserver::swagger::SwaggerExample;
use axum::response::{IntoResponse, Response};
use axum::Json;
use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error, Serialize, ToSchema)]
pub enum ServerError {
    #[error("not found error: {0}")]
    NotFound(String),
    #[error("internal service error: {0}")]
    InternalError(String),
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
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

impl From<StorageError> for ServerError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::ServiceUnavailable(err) => {
                ServerError::ServiceUnavailable(err.to_string())
            }
            StorageError::RequestTimeout(err) => ServerError::InternalError(err.to_string()),
            StorageError::ServiceError(err) => ServerError::InternalError(err.to_string()),
            StorageError::RuntimeError(err) => ServerError::InternalError(err.to_string()),
            StorageError::SerdeError(err) => ServerError::InternalError(err.to_string()),
            StorageError::NotFound(err) => ServerError::NotFound(err.to_string()),
            StorageError::ComputeEmbeddings(err) => ServerError::InternalError(err.to_string()),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (msg, status) = self.status_code();
        let mut resp = Json(ErrorResponse {
            message: msg.to_string(),
        })
        .into_response();

        *resp.status_mut() = status;
        resp
    }
}

impl SwaggerExample for ServerError {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        match value {
            None => ServerError::ServiceUnavailable("service unavailable".to_owned()),
            Some(msg) => ServerError::InternalError(msg.to_owned()),
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
