use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use doc_search_core::domain::searcher::SearchError;
use doc_search_core::domain::storage::StorageError;
use serde_derive::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

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
            StorageError::AuthenticationFailed(err) => {
                ServerError::AuthenticationFailed(err.to_string())
            }
            StorageError::IndexNotFound(err) => ServerError::NotFound(err.to_string()),
            StorageError::DocumentAlreadyExists(err) => {
                ServerError::Conflict(err.to_string().to_string())
            }
            StorageError::DocumentNotFound(err) => ServerError::NotFound(err.to_string()),
            StorageError::InternalError(err) => ServerError::InternalError(err.to_string()),
            StorageError::ValidationError(err) => ServerError::IncorrectInputForm(err.to_string()),
            StorageError::CantSplitLargeDocuments(err) => {
                ServerError::InternalError(err.to_string())
            }
            StorageError::ConnectionError(err) => ServerError::InternalError(err.to_string()),
            StorageError::UnknownError(err) => ServerError::InternalError(err.to_string()),
        }
    }
}

impl From<SearchError> for ServerError {
    fn from(err: SearchError) -> Self {
        match err {
            SearchError::AuthenticationFailed(err) => {
                ServerError::AuthenticationFailed(err.to_string())
            }
            SearchError::IndexNotFound(err) => ServerError::NotFound(err.to_string()),
            SearchError::InternalError(err) => ServerError::InternalError(err.to_string()),
            SearchError::ValidationError(err) => ServerError::IncorrectInputForm(err.to_string()),
            SearchError::ConnectionError(err) => ServerError::InternalError(err.to_string()),
            SearchError::UnknownError(err) => ServerError::InternalError(err.to_string()),
            SearchError::ServiceError(err) => ServerError::InternalError(err.to_string()),
        }
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
            ServerError::ServerUnavailable => (StatusCode::SERVICE_UNAVAILABLE, UNAVAILABLE_SERVER),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            status: u16,
            message: String,
        }

        let (status, msg) = self.status_code();
        tracing::error!(status=%status, msg=%msg, "error response");
        let response = ErrorResponse {
            status: status.as_u16(),
            message: msg.to_string(),
        };
        let mut resp = Json(response).into_response();
        *resp.status_mut() = status;
        resp
    }
}

#[derive(Serialize, ToSchema)]
pub struct Success {
    #[schema(example = 200)]
    pub status: u16,
    #[schema(example = "ok")]
    pub message: String,
}

impl Default for Success {
    fn default() -> Self {
        let status_code = StatusCode::OK;
        Success {
            status: status_code.as_u16(),
            message: "ok".to_string(),
        }
    }
}

impl Success {
    pub fn new(status: u16, message: &str) -> Self {
        let message = message.to_string();
        Success { status, message }
    }
}
