use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use elasticsearch::Error;
use serde::Serialize;
use thiserror::Error;

pub type WebResponse<T> = Result<T, WebError>;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Unknown error: {0}")]
    RuntimeError(String),
    #[error("Error while getting cluster: {0}")]
    GetClusterError(String),
    #[error("Error while creating cluster: {0}")]
    CreateClusterError(String),
    #[error("Error while deleting cluster: {0}")]
    DeletingClusterError(String),
    #[error("Error while getting bucket: {0}")]
    GetBucketError(String),
    #[error("Error while creating bucket: {0}")]
    CreateBucketError(String),
    #[error("Error while deleting bucket: {0}")]
    DeleteBucketError(String),
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
    #[error("Response error: {0}")]
    ResponseError(String),
    #[error("Failed while parsing bucket: {0}")]
    BucketParsingError(String),
}

impl WebError {
    pub fn name(&self) -> String {
        match self {
            WebError::SearchError(_) => "SearchError",
            WebError::ResponseError(_) => "ResponseError",
            WebError::GetBucketError(_) => "GetBucketError",
            WebError::GetClusterError(_) => "GetClusterError",
            WebError::GetDocumentError(_) => "GetDocumentError",
            WebError::CreateDocumentError(_) => "CreateDocumentError",
            WebError::UpdateDocumentError(_) => "UpdateDocumentError",
            WebError::DeleteDocumentError(_) => "DeleteDocumentError",
            WebError::DocumentSerializingError(_) => "DocumentSerializingError",
            WebError::BucketParsingError(_) => "BucketParsingError",
            _ => "RuntimeError",
        }
        .to_string()
    }
}

impl From<elasticsearch::Error> for WebError {
    fn from(value: Error) -> Self {
        WebError::ResponseError(value.to_string())
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::SearchError(_) => StatusCode::BAD_REQUEST,
            WebError::ResponseError(_) => StatusCode::BAD_REQUEST,
            WebError::GetBucketError(_) => StatusCode::BAD_REQUEST,
            WebError::GetClusterError(_) => StatusCode::BAD_REQUEST,
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
