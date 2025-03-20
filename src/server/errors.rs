use axum::response::{IntoResponse, Response};
use axum::Json;
use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::engine::error::SearcherError;
use crate::server::swagger::SwaggerExample;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error, Serialize, ToSchema)]
pub enum ServerError {
    #[error("not found error: {0}")]
    NotFound(String),
    #[error("worker {0} is launched")]
    Launched(String),
    #[error("internal service error: {0}")]
    InternalError(String),
    #[error("searcher error: {0}")]
    SearcherError(String),
    #[error("service unavailable")]
    ServiceUnavailable,
}

impl ServerError {
    pub fn status_code(&self) -> (String, StatusCode) {
        match self {
            ServerError::NotFound(msg) => (msg.to_owned(), StatusCode::NOT_FOUND),
            ServerError::Launched(msg) => (msg.to_owned(), StatusCode::CONFLICT),
            ServerError::InternalError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::SearcherError(err) => (err.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::ServiceUnavailable => {
                ("service unavailable".to_owned(), StatusCode::SERVICE_UNAVAILABLE)
            }
        }
    }
}

impl From<SearcherError> for ServerError {
    fn from(err: SearcherError) -> Self {
        ServerError::SearcherError(err.to_string())
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
            None => ServerError::ServiceUnavailable,
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

impl SwaggerExample for Success {
    type Example = Self;

    fn example(_value: Option<&str>) -> Self::Example {
        Success::default()
    }
}
