use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use elasticsearch::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type WebResponse<T> = Result<T, WebError>;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Error while getting cluster: {0}")]
    GetCluster(String),
    #[error("Error while creating cluster: {0}")]
    CreateCluster(String),
    #[error("Error while deleting cluster: {0}")]
    DeletingCluster(String),
    #[error("Error while getting bucket: {0}")]
    GetBucket(String),
    #[error("Error while creating bucket: {0}")]
    CreateBucket(String),
    #[error("Error while deleting bucket: {0}")]
    DeleteBucket(String),
    #[error("Error elastic response: {0}")]
    GetDocument(String),
    #[error("Failed while creating document: {0}")]
    CreateDocument(String),
    #[error("Failed while updating document: {0}")]
    UpdateDocument(String),
    #[error("Failed while deleting document: {0}")]
    DeleteDocument(String),
    #[error("Failed while serializing document: {0}")]
    DocumentSerializing(String),
    #[error("Failed while searching: {0}")]
    SearchFailed(String),
    #[error("Response error: {0}")]
    ResponseError(String),
    #[error("Failed while parsing bucket: {0}")]
    BucketParsing(String),
}

impl WebError {
    pub fn name(&self) -> String {
        match self {
            WebError::SearchFailed(_) => "SearchError",
            WebError::ResponseError(_) => "ResponseError",
            WebError::GetBucket(_) => "GetBucketError",
            WebError::GetCluster(_) => "GetClusterError",
            WebError::GetDocument(_) => "GetDocumentError",
            WebError::CreateDocument(_) => "CreateDocumentError",
            WebError::UpdateDocument(_) => "UpdateDocumentError",
            WebError::DeleteDocument(_) => "DeleteDocumentError",
            WebError::DocumentSerializing(_) => "DocumentSerializingError",
            WebError::BucketParsing(_) => "BucketParsingError",
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
            WebError::SearchFailed(_) => StatusCode::BAD_REQUEST,
            WebError::ResponseError(_) => StatusCode::BAD_REQUEST,
            WebError::GetBucket(_) => StatusCode::BAD_REQUEST,
            WebError::GetCluster(_) => StatusCode::BAD_REQUEST,
            WebError::GetDocument(_) => StatusCode::BAD_REQUEST,
            WebError::CreateDocument(_) => StatusCode::BAD_REQUEST,
            WebError::UpdateDocument(_) => StatusCode::BAD_REQUEST,
            WebError::DeleteDocument(_) => StatusCode::BAD_REQUEST,
            WebError::DocumentSerializing(_) => StatusCode::BAD_REQUEST,
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

#[derive(Serialize)]
pub struct SuccessfulResponse {
    code: u16,
    message: String,
}

impl SuccessfulResponse {
    pub fn ok_response(msg: &str) -> HttpResponse {
        let status_code = StatusCode::OK;
        let response = SuccessfulResponse {
            code: status_code.as_u16(),
            message: msg.to_string(),
        };

        HttpResponse::build(status_code).json(response)
    }
}
