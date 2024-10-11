use crate::searcher::models::Paginated;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use elasticsearch::http::response::Exception;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::Error;
use thiserror::Error;
use utoipa::ToSchema;

pub type WebResult<T> = Result<T, WebError>;
pub type JsonResponse<T> = Result<web::Json<T>, WebError>;
pub type PaginateResponse<T> = JsonResponse<Paginated<T>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct WebErrorEntity {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
}

impl Display for WebErrorEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let self_data = &self.description;
        write!(f, "{}", self_data.clone())
    }
}

impl WebErrorEntity {
    pub fn new(msg: String) -> WebErrorEntity {
        WebErrorEntity {
            description: msg.to_owned(),
            attachments: None,
        }
    }

    pub fn with_attachments(msg: String, attach: Vec<String>) -> WebErrorEntity {
        WebErrorEntity {
            description: msg.to_owned(),
            attachments: Some(attach),
        }
    }
}

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Failed to get all clusters: {0}")]
    GetClusters(WebErrorEntity),
    #[error("Failed to get cluster details: {0}")]
    GetCluster(WebErrorEntity),
    #[error("Failed to create new cluster: {0}")]
    CreateCluster(WebErrorEntity),
    #[error("Failed to delete cluster: {0}")]
    DeleteCluster(WebErrorEntity),
    #[error("Failed to get all folders: {0}")]
    GetFolders(WebErrorEntity),
    #[error("Failed to get folder details: {0}")]
    GetFolder(WebErrorEntity),
    #[error("Failed to create new folder: {0}")]
    CreateFolder(WebErrorEntity),
    #[error("Failed to delete folder: {0}")]
    DeleteFolder(WebErrorEntity),
    #[error("Failed to get document: {0}")]
    GetDocument(WebErrorEntity),
    #[error("Failed to create new document: {0}")]
    CreateDocument(WebErrorEntity),
    #[error("Failed to delete document: {0}")]
    DeleteDocument(WebErrorEntity),
    #[error("Failed to update document: {0}")]
    UpdateDocument(WebErrorEntity),
    #[error("Failed to move documents to folder: {0}")]
    MoveDocuments(WebErrorEntity),
    #[error("Failed to (de)serialize object: {0}")]
    SerdeError(WebErrorEntity),
    #[error("Failed while searching: {0}")]
    SearchError(WebErrorEntity),
    #[error("Error response from searcher service: {0}")]
    SearchServiceError(WebErrorEntity),
    #[error("Failed to upload file: {0}")]
    UploadFileError(WebErrorEntity),
    #[error("Failed while paginating: {0}")]
    PaginationError(WebErrorEntity),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(WebErrorEntity),
    #[error("Response error: {0}")]
    UnknownError(WebErrorEntity),
    #[error("Continues executing: {0}")]
    ResponseContinues(WebErrorEntity),
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
    pub fn attachments(&self) -> Option<Vec<String>> {
        match self {
            WebError::GetClusters(attach) => attach.attachments.clone(),
            WebError::GetCluster(attach) => attach.attachments.clone(),
            WebError::CreateCluster(attach) => attach.attachments.clone(),
            WebError::DeleteCluster(attach) => attach.attachments.clone(),
            WebError::GetFolders(attach) => attach.attachments.clone(),
            WebError::GetFolder(attach) => attach.attachments.clone(),
            WebError::CreateFolder(attach) => attach.attachments.clone(),
            WebError::DeleteFolder(attach) => attach.attachments.clone(),
            WebError::GetDocument(attach) => attach.attachments.clone(),
            WebError::CreateDocument(attach) => attach.attachments.clone(),
            WebError::DeleteDocument(attach) => attach.attachments.clone(),
            WebError::UpdateDocument(attach) => attach.attachments.clone(),
            WebError::MoveDocuments(attach) => attach.attachments.clone(),
            WebError::SerdeError(attach) => attach.attachments.clone(),
            WebError::SearchError(attach) => attach.attachments.clone(),
            WebError::SearchServiceError(attach) => attach.attachments.clone(),
            WebError::UploadFileError(attach) => attach.attachments.clone(),
            WebError::PaginationError(attach) => attach.attachments.clone(),
            WebError::ServiceUnavailable(attach) => attach.attachments.clone(),
            WebError::ResponseContinues(attach) => attach.attachments.clone(),
            WebError::UnknownError(attach) => attach.attachments.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub(crate) struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
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
            attachments: self.attachments(),
        };

        HttpResponse::build(status_code).json(response)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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
        tracing::error!("{}", err_msg);
        WebError::UnknownError(WebErrorEntity {
            description: err_msg.to_string(),
            attachments: None,
        })
    }
}

impl From<elasticsearch::Error> for WebError {
    fn from(value: elasticsearch::Error) -> Self {
        let err_msg = value.to_string();
        tracing::error!("{}", err_msg.as_str());
        WebError::SearchServiceError(WebErrorEntity {
            description: err_msg.to_string(),
            attachments: None,
        })
    }
}

impl From<serde_json::Error> for WebError {
    fn from(value: serde_json::Error) -> Self {
        let err_msg = value.to_string();
        tracing::error!("{}", err_msg.as_str());
        WebError::UnknownError(WebErrorEntity {
            description: err_msg.to_string(),
            attachments: None,
        })
    }
}

impl From<std::io::Error> for WebError {
    fn from(value: Error) -> Self {
        let err_msg = value.to_string();
        tracing::error!("{}", err_msg.as_str());
        WebError::UploadFileError(WebErrorEntity {
            description: err_msg.to_string(),
            attachments: None,
        })
    }
}

impl From<reqwest::Error> for WebError {
    fn from(value: reqwest::Error) -> Self {
        let err_msg = value.to_string();
        tracing::error!("{}", err_msg.as_str());
        WebError::ServiceUnavailable(WebErrorEntity {
            description: err_msg.to_string(),
            attachments: None,
        })
    }
}
