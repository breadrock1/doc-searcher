use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use elasticsearch::Error;
use serde::Serialize;
use thiserror::Error;

pub type WebResponse<T> = Result<T, WebError>;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Unknown error: {0}")]
    RuntimeError(String),
    #[error("Error while getting cluster: {0}")]
    GetClusterError(String),
    #[error("Error while getting bucket: {0}")]
    GetBucketError(String),
    #[error("Error elastic response: {0}")]
    GetDocumentError(String),
    #[error("Failed while creating document: {0}")]
    CreateDocumentError(String),
    #[error("Failed while updating document: {0}")]
    UpdateDocumentError(String),
    #[error("Failed while deleting document: {0}")]
    DeleteDocumentError(String),
    #[error("Failed while serializing document: {0}")]
    DocumentSerializingError(String),
    #[error("Failed while searching: {0}")]
    SearchError(String),
}

impl WebError {
    pub fn new(msg: String) -> Self {
        WebError::RuntimeError(msg)
    }

    pub fn name(&self) -> String {
        match self {
            WebError::SearchError(_) => "SearchError",
            WebError::GetClusterError(_) => "GetClusterError",
            WebError::GetBucketError(_) => "GetBucketError",
            WebError::GetDocumentError(_) => "GetDocumentError",
            WebError::CreateDocumentError(_) => "CreateDocumentError",
            WebError::UpdateDocumentError(_) => "UpdateDocumentError",
            WebError::DeleteDocumentError(_) => "DeleteDocumentError",
            WebError::DocumentSerializingError(_) => "DocumentSerializingError",
            _ => "RuntimeError",
        }
        .to_string()
    }
}

impl From<elasticsearch::Error> for WebError {
    fn from(value: Error) -> Self {
        WebError::new(value.to_string())
    }
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::SearchError(_) => StatusCode::BAD_REQUEST,
            WebError::GetClusterError(_) => StatusCode::BAD_REQUEST,
            WebError::GetBucketError(_) => StatusCode::BAD_REQUEST,
            WebError::GetDocumentError(_) => StatusCode::BAD_REQUEST,
            WebError::CreateDocumentError(_) => StatusCode::BAD_REQUEST,
            WebError::UpdateDocumentError(_) => StatusCode::BAD_REQUEST,
            WebError::DeleteDocumentError(_) => StatusCode::BAD_REQUEST,
            WebError::DocumentSerializingError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };

        HttpResponse::build(status_code).json(response)
    }
}
