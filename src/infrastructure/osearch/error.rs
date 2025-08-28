#![allow(dead_code)]

use anyhow::anyhow;
use gset::Getset;
use opensearch::http::response::Response;
use reqwest::StatusCode;
use serde_derive::Deserialize;
use thiserror::Error;

use crate::application::services::storage::error::StorageError;

const UNKNOWN_ERROR_TYPE: &str = "unknown";

#[derive(Debug, Error)]
pub enum OSearchError {
    #[error("authentication failed: {0}")]
    AuthenticationFailed(anyhow::Error),
    #[error("index has not been founded: {0}")]
    IndexNotFound(anyhow::Error),
    #[error("document has not been found: {0}")]
    DocumentNotFound(anyhow::Error),
    #[error("document already exists: {0}")]
    DocumentAlreadyExists(anyhow::Error),
    #[error("validation error: {0}")]
    ValidationError(anyhow::Error),
    #[error("execution error: {0}")]
    ExecutionError(anyhow::Error),
    #[error("connection error: {0}")]
    ConnectionError(anyhow::Error),
    #[error("returned undeclared error from opensearch: {0}")]
    UndeclaredError(anyhow::Error),
}

impl OSearchError {
    pub async fn from_response(response: Response) -> OSearchError {
        let status = response.status_code();
        tracing::debug!(response=?response, "returned error response");
        let data = match response.text().await {
            Ok(data) => data,
            Err(err) => {
                let err = anyhow!(err);
                return Self::extract_from_http_status(status, err)
            }
        };

        if let Ok(err) = serde_json::from_str::<ResponseError>(&data) {
            return Self::extract_error(err)
        }

        if let Ok(err) = serde_json::from_str::<NotFoundDocument>(&data) {
            let err= anyhow!("document [{}] not found in index: [{}]", err._id, err._index);
            return OSearchError::DocumentNotFound(err)
        };

        let err = anyhow!("undeclared error: {data}");
        Self::extract_from_http_status(status, err)
    }

    fn extract_error(err: ResponseError) -> OSearchError {
        let msg = err
            .details
            .root_cause
            .iter()
            .map(|it| it.reason.as_str())
            .collect::<Vec<&str>>()
            .join(": ");

        let _err = anyhow!(msg);
        let details = &err.details;
        tracing::warn!(details=?details, "details data");
        match details.error_type.as_str() {
            "index_not_found_exception" => OSearchError::IndexNotFound(_err),
            "document_missing_exception" | "document_not_found" => {
                OSearchError::DocumentNotFound(_err)
            },
            "resource_already_exists_exception" | "version_conflict_engine_exception" => {
                OSearchError::DocumentAlreadyExists(_err)
            },
            "security_exception" | "authentication_exception" => {
                OSearchError::AuthenticationFailed(_err)
            },
            "validation_exception" | "illegal_argument_exception" => {
                OSearchError::ValidationError(_err)
            },
            "search_phase_execution_exception" | "search_context_missing_exception" => {
                OSearchError::ExecutionError(_err)
            },
            _ => {
                let status_code = StatusCode::from_u16(err.status)
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

                Self::extract_from_http_status(status_code, _err)
            }
        }
    }

    fn extract_from_http_status(status: StatusCode, err: anyhow::Error) -> OSearchError {
        match status {
            StatusCode::NOT_FOUND => OSearchError::IndexNotFound(err),
            StatusCode::BAD_REQUEST => OSearchError::ValidationError(err),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                OSearchError::AuthenticationFailed(err)
            },
            StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => {
                OSearchError::ExecutionError(err)
            },
            StatusCode::SERVICE_UNAVAILABLE => OSearchError::ConnectionError(err),
            _ => OSearchError::UndeclaredError(err),
        }
    }
}

impl From<OSearchError> for StorageError {
    fn from(err: OSearchError) -> Self {
        match err {
            OSearchError::AuthenticationFailed(err) => {
                StorageError::AuthenticationFailed(err)
            }
            OSearchError::IndexNotFound(err) => {
                StorageError::IndexNotFound(err)
            }
            OSearchError::DocumentNotFound(err) => {
                StorageError::DocumentNotFound(err)
            }
            OSearchError::DocumentAlreadyExists(err) => {
                StorageError::DocumentAlreadyExists(err)
            }
            OSearchError::ValidationError(err) => {
                StorageError::ValidationError(err)
            }
            OSearchError::ExecutionError(err) => {
                StorageError::ServiceError(err)
            }
            OSearchError::ConnectionError(err) => {
                StorageError::InternalError(err)
            }
            OSearchError::UndeclaredError(err) => {
                StorageError::InternalError(err)
            }
        }
    }
}

#[derive(Debug, Getset, Deserialize)]
struct NotFoundDocument {
    #[getset(get, vis = "pub")]
    _index: String,
    #[getset(get, vis = "pub")]
    _id: String,
    #[getset(get_copy, vis = "pub")]
    found: Option<bool>,
    #[getset(get, vis = "pub")]
    result: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResponseError {
    status: u16,
    #[serde(rename = "error")]
    details: ErrorDetails,
}

#[derive(Debug, Deserialize)]
struct ErrorDetails {
    root_cause: Vec<ErrorRootCause>,
    #[serde(rename = "type")]
    error_type: String,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct ErrorRootCause {
    #[serde(rename = "type")]
    error_type: String,
    reason: String,
}

impl ResponseError {
    fn from_error(err: opensearch::Error) -> Self {
        let status_code = err
            .status_code()
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let err_root_case = ErrorRootCause {
            error_type: UNKNOWN_ERROR_TYPE.to_string(),
            reason: err.to_string(),
        };

        let error_detail = ErrorDetails {
            root_cause: vec![err_root_case],
            error_type: UNKNOWN_ERROR_TYPE.to_string(),
            reason: err.to_string(),
        };

        ResponseError {
            details: error_detail,
            status: status_code.as_u16(),
        }
    }
}

impl From<opensearch::Error> for StorageError {
    fn from(err: opensearch::Error) -> Self {
        let err = ResponseError::from_error(err);
        let err = OSearchError::extract_error(err);
        StorageError::from(err)
    }
}
