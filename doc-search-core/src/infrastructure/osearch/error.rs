#![allow(dead_code)]

use anyhow::anyhow;
use gset::Getset;
use opensearch::http::response::Response;
use reqwest::StatusCode;
use serde_derive::Deserialize;
use thiserror::Error;

use crate::domain::searcher::SearchError;
use crate::domain::storage::StorageError;

const UNKNOWN_ERROR_TYPE: &str = "unknown";

pub type OSearchResult<T> = Result<T, OSearchError>;

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
    #[error("failed to build query: {0}")]
    BuildQueryError(anyhow::Error),
    #[error("execution error: {0}")]
    ExecutionError(anyhow::Error),
    #[error("connection error: {0}")]
    ConnectionError(anyhow::Error),
    #[error("returned undeclared error from opensearch: {0}")]
    UndeclaredError(anyhow::Error),
}

impl From<OSearchError> for StorageError {
    fn from(err: OSearchError) -> Self {
        match err {
            OSearchError::AuthenticationFailed(err) => StorageError::AuthenticationFailed(err),
            OSearchError::IndexNotFound(err) => StorageError::IndexNotFound(err),
            OSearchError::DocumentNotFound(err) => StorageError::DocumentNotFound(err),
            OSearchError::DocumentAlreadyExists(err) => StorageError::DocumentAlreadyExists(err),
            OSearchError::ValidationError(err) => StorageError::ValidationError(err),
            OSearchError::BuildQueryError(err) => StorageError::InternalError(err),
            OSearchError::ExecutionError(err) => StorageError::InternalError(err),
            OSearchError::ConnectionError(err) => StorageError::InternalError(err),
            OSearchError::UndeclaredError(err) => StorageError::InternalError(err),
        }
    }
}

impl From<OSearchError> for SearchError {
    fn from(err: OSearchError) -> Self {
        match err {
            OSearchError::AuthenticationFailed(err) => SearchError::AuthenticationFailed(err),
            OSearchError::IndexNotFound(err) => SearchError::IndexNotFound(err),
            OSearchError::DocumentNotFound(err) => SearchError::InternalError(err),
            OSearchError::DocumentAlreadyExists(err) => SearchError::InternalError(err),
            OSearchError::ValidationError(err) => SearchError::ValidationError(err),
            OSearchError::BuildQueryError(err) => SearchError::InternalError(err),
            OSearchError::ExecutionError(err) => SearchError::ServiceError(err),
            OSearchError::ConnectionError(err) => SearchError::ConnectionError(err),
            OSearchError::UndeclaredError(err) => SearchError::UnknownError(err),
        }
    }
}

impl OSearchError {
    pub async fn from_response(response: Response) -> OSearchError {
        let status = response.status_code();
        tracing::debug!(?response, "returned error response");
        let data = match response.text().await {
            Ok(data) => data,
            Err(err) => {
                let err = anyhow!(err);
                return Self::extract_from_http_status(status, err);
            }
        };

        if let Ok(err) = serde_json::from_str::<ResponseError>(&data) {
            return Self::extract_error(err);
        }

        if let Ok(err) = serde_json::from_str::<NotFoundDocument>(&data) {
            let err = anyhow!(
                "document [{}] not found in index: [{}]",
                err._id,
                err._index
            );
            return OSearchError::DocumentNotFound(err);
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
        match details.error_type.as_str() {
            "index_not_found_exception" => OSearchError::IndexNotFound(_err),
            "document_missing_exception" | "document_not_found" => {
                OSearchError::DocumentNotFound(_err)
            }
            "resource_already_exists_exception" | "version_conflict_engine_exception" => {
                OSearchError::DocumentAlreadyExists(_err)
            }
            "security_exception" | "authentication_exception" => {
                OSearchError::AuthenticationFailed(_err)
            }
            "validation_exception" | "illegal_argument_exception" => {
                OSearchError::ValidationError(_err)
            }
            "search_phase_execution_exception" | "search_context_missing_exception" => {
                OSearchError::ExecutionError(_err)
            }
            _ => {
                let status_code =
                    StatusCode::from_u16(err.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

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
            }
            StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => {
                OSearchError::ExecutionError(err)
            }
            StatusCode::SERVICE_UNAVAILABLE => OSearchError::ConnectionError(err),
            _ => OSearchError::UndeclaredError(err),
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use rstest::rstest;

    #[rstest]
    #[case::index_not_found("index_not_found_exception", "IndexNotFound")]
    #[case::doc_missing("document_missing_exception", "DocumentNotFound")]
    #[case::doc_not_found("document_not_found", "DocumentNotFound")]
    #[case::already_exists("resource_already_exists_exception", "DocumentAlreadyExists")]
    #[case::version_conflict("version_conflict_engine_exception", "DocumentAlreadyExists")]
    #[case::security("security_exception", "AuthenticationFailed")]
    #[case::auth("authentication_exception", "AuthenticationFailed")]
    #[case::validation("validation_exception", "ValidationError")]
    #[case::illegal_argument("illegal_argument_exception", "ValidationError")]
    #[case::search_phase("search_phase_execution_exception", "ExecutionError")]
    #[case::search_context("search_context_missing_exception", "ExecutionError")]
    fn test_extract_error_by_type(#[case] error_type: &str, #[case] expected: &str) {
        let err = OSearchError::extract_error(response_error(error_type, 500));
        assert_eq!(expected, variant(err));
    }

    #[rstest]
    #[case::not_found(404, "IndexNotFound")]
    #[case::bad_request(400, "ValidationError")]
    #[case::unauthorized(401, "AuthenticationFailed")]
    #[case::forbidden(403, "AuthenticationFailed")]
    #[case::request_timeout(408, "ExecutionError")]
    #[case::gateway_timeout(504, "ExecutionError")]
    #[case::unavailable(503, "ConnectionError")]
    #[case::undeclared(500, "UndeclaredError")]
    fn test_extract_error_unknown_type_by_status(#[case] status: u16, #[case] expected: &str) {
        let err = OSearchError::extract_error(response_error("some_unknown", status));
        assert_eq!(expected, variant(err));
    }

    #[rstest]
    #[case::not_found(StatusCode::NOT_FOUND, "IndexNotFound")]
    #[case::bad_request(StatusCode::BAD_REQUEST, "ValidationError")]
    #[case::unauthorized(StatusCode::UNAUTHORIZED, "AuthenticationFailed")]
    #[case::forbidden(StatusCode::FORBIDDEN, "AuthenticationFailed")]
    #[case::request_timeout(StatusCode::REQUEST_TIMEOUT, "ExecutionError")]
    #[case::gateway_timeout(StatusCode::GATEWAY_TIMEOUT, "ExecutionError")]
    #[case::unavailable(StatusCode::SERVICE_UNAVAILABLE, "ConnectionError")]
    #[case::undeclared(StatusCode::INTERNAL_SERVER_ERROR, "UndeclaredError")]
    fn test_extract_from_http_status(#[case] status: StatusCode, #[case] expected: &str) {
        let err = OSearchError::extract_from_http_status(status, anyhow!("boom"));
        assert_eq!(expected, variant(err));
    }

    fn response_error(error_type: &str, status: u16) -> ResponseError {
        ResponseError {
            status,
            details: ErrorDetails {
                root_cause: vec![ErrorRootCause {
                    error_type: error_type.to_string(),
                    reason: "boom".to_string(),
                }],
                error_type: error_type.to_string(),
                reason: "boom".to_string(),
            },
        }
    }

    fn variant(err: OSearchError) -> &'static str {
        match err {
            OSearchError::AuthenticationFailed(_) => "AuthenticationFailed",
            OSearchError::IndexNotFound(_) => "IndexNotFound",
            OSearchError::DocumentNotFound(_) => "DocumentNotFound",
            OSearchError::DocumentAlreadyExists(_) => "DocumentAlreadyExists",
            OSearchError::ValidationError(_) => "ValidationError",
            OSearchError::BuildQueryError(_) => "BuildQueryError",
            OSearchError::ExecutionError(_) => "ExecutionError",
            OSearchError::ConnectionError(_) => "ConnectionError",
            OSearchError::UndeclaredError(_) => "UndeclaredError",
        }
    }
}
