use crate::forms::pagination::pagination::Paginated;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use elasticsearch::http::response::Exception;
use serde::{Deserialize, Serialize};
use std::io::Error;
use thiserror::Error;
use utoipa::ToSchema;

pub(crate) type WebResult<T> = Result<T, WebError>;
pub(crate) type JsonResponse<T> = Result<web::Json<T>, WebError>;
pub(crate) type PaginateResponse<T> = JsonResponse<Paginated<T>>;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Failed to get all clusters: {0}")]
    GetClusters(String),
    #[error("Failed to get cluster details: {0}")]
    GetCluster(String),
    #[error("Failed to create new cluster: {0}")]
    CreateCluster(String),
    #[error("Failed to delete cluster: {0}")]
    DeleteCluster(String),
    #[error("Failed to get all folders: {0}")]
    GetFolders(String),
    #[error("Failed to get folder details: {0}")]
    GetFolder(String),
    #[error("Failed to create new folder: {0}")]
    CreateFolder(String),
    #[error("Failed to delete folder: {0}")]
    DeleteFolder(String),
    #[error("Failed to get document: {0}")]
    GetDocument(String),
    #[error("Failed to create new document: {0}")]
    CreateDocument(String),
    #[error("Failed to delete document: {0}")]
    DeleteDocument(String),
    #[error("Failed to update document: {0}")]
    UpdateDocument(String),
    #[error("Failed to move documents to folder: {0}")]
    MoveDocuments(String),
    #[error("Failed to (de)serialize object: {0}")]
    SerdeError(String),
    #[error("Failed while searching: {0}")]
    SearchError(String),
    #[error("Error response from searcher service: {0}")]
    SearchServiceError(String),
    #[error("Failed to upload file: {0}")]
    UploadFileError(String),
    #[error("Failed while paginating: {0}")]
    PaginationError(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Response error: {0}")]
    UnknownError(String),
    #[error("Continues executing: {0}")]
    ResponseContinues(String),
}

impl WebError {
    pub fn name(&self) -> &str {
        match self {
            WebError::GetClusters(_) => "Get clusters error",
            WebError::GetCluster(_) => "Get cluster error",
            WebError::CreateCluster(_) => "Create cluster error",
            WebError::DeleteCluster(_) => "Delete cluster error",
            WebError::GetFolders(_) => "Get folders error",
            WebError::GetFolder(_) => "Get folder error",
            WebError::CreateFolder(_) => "Create folder error",
            WebError::DeleteFolder(_) => "Delete folder error",
            WebError::GetDocument(_) => "Get document error",
            WebError::CreateDocument(_) => "Create document error",
            WebError::DeleteDocument(_) => "Delete document error",
            WebError::UpdateDocument(_) => "Update document error",
            WebError::MoveDocuments(_) => "Move documents error",
            WebError::SerdeError(_) => "Serde error",
            WebError::SearchError(_) => "Search data error",
            WebError::SearchServiceError(_) => "Search server error",
            WebError::UploadFileError(_) => "Upload file error",
            WebError::PaginationError(_) => "Pagination error",
            WebError::ServiceUnavailable(_) => "Service unavailable",
            WebError::ResponseContinues(_) => "Processing...",
            WebError::UnknownError(_) => "Runtime error",
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub(crate) struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::ResponseContinues(_) => StatusCode::PROCESSING,
            WebError::SerdeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name().to_string(),
        };

        HttpResponse::build(status_code).json(response)
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Successful {
    pub code: u16,
    pub message: String,
}

impl Successful {
    pub fn new(code: u16, msg: &str) -> Self {
        let message = msg.to_string();
        Successful { code, message }
    }
    pub fn success(msg: &str) -> Self {
        Successful {
            code: 200u16,
            message: msg.to_string(),
        }
    }
    pub fn is_success(&self) -> bool {
        self.code == 200
    }
    pub fn get_msg(&self) -> &str {
        self.message.as_str()
    }
}

impl From<Exception> for WebError {
    fn from(value: Exception) -> Self {
        let err_msg = value.error().reason().unwrap();
        log::error!("{}", err_msg);
        WebError::UnknownError(err_msg.to_string())
    }
}

impl From<elasticsearch::Error> for WebError {
    fn from(value: elasticsearch::Error) -> Self {
        let err_msg = value.to_string();
        log::error!("{}", err_msg.as_str());
        WebError::SearchServiceError(err_msg)
    }
}

impl From<serde_json::Error> for WebError {
    fn from(value: serde_json::Error) -> Self {
        let err_msg = value.to_string();
        log::error!("{}", err_msg.as_str());
        WebError::UnknownError(err_msg)
    }
}

impl From<std::io::Error> for WebError {
    fn from(value: Error) -> Self {
        let err_msg = value.to_string();
        log::error!("{}", err_msg.as_str());
        WebError::UploadFileError(err_msg)
    }
}

impl From<reqwest::Error> for WebError {
    fn from(value: reqwest::Error) -> Self {
        let err_msg = value.to_string();
        log::error!("{}", err_msg.as_str());
        WebError::ServiceUnavailable(err_msg)
    }
}
