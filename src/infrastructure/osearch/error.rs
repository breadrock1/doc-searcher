#![allow(dead_code)]

use opensearch::http::response::Response;
use reqwest::StatusCode;
use serde_derive::Deserialize;

use crate::application::services::storage::error::StorageError;

#[derive(Debug, Deserialize)]
pub struct OpenSearchError {
    error: OpenSearchErrorDetail,
    status: u16,
}

#[derive(Debug, Deserialize)]
struct OpenSearchErrorDetail {
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

impl OpenSearchError {
    pub async fn from_response(response: Response) -> StorageError {
        let status = response.status_code();
        match response.json::<OpenSearchError>().await {
            Err(err) => StorageError::RuntimeError(err.to_string()),
            Ok(err) => {
                let msg = err
                    .error
                    .root_cause
                    .iter()
                    .map(|it| it.reason.as_str())
                    .collect::<Vec<&str>>()
                    .join(": ");

                if status.as_u16() == StatusCode::CONFLICT {
                    let msg = format!("document already exists: {msg}");
                    let err = anyhow::Error::msg(msg);
                    return StorageError::AlreadyExists(err);
                }

                StorageError::ServiceError(anyhow::Error::msg(msg))
            }
        }
    }
}

impl From<opensearch::Error> for StorageError {
    fn from(err: opensearch::Error) -> Self {
        StorageError::ServiceError(anyhow::Error::from(err))
    }
}
