use anyhow::anyhow;
use qdrant_client::QdrantError;

use crate::application::services::storage::StorageError;

impl From<QdrantError> for StorageError {
    fn from(err: QdrantError) -> Self {
        match err {
            QdrantError::ResponseError { status } => {
                let err = anyhow!("response error: {status}");
                StorageError::InternalError(err)
            }
            QdrantError::ResourceExhaustedError { status, .. } => {
                let err = anyhow!("resource exhausted: {status}");
                StorageError::InternalError(err)
            }
            QdrantError::ConversionError(msg) => {
                let err = anyhow!("conversion exhausted: {msg}");
                StorageError::InternalError(err)
            }
            QdrantError::InvalidUri(uri) => {
                let err = anyhow!("invalid URI: {uri}");
                StorageError::InternalError(err)
            }
            QdrantError::NoSnapshotFound(msg) => {
                let err = anyhow!("no snapshot found: {msg}");
                StorageError::InternalError(err)
            }
            QdrantError::Io(err) => {
                let err = anyhow!("IO error: {err}");
                StorageError::InternalError(err)
            }
            QdrantError::Reqwest(err) => {
                let err = anyhow!("request err: {err}");
                StorageError::InternalError(err)
            }
            QdrantError::JsonToPayload(value) => {
                let err = anyhow!("invalid data: {value}");
                StorageError::InternalError(err)
            }
        }
    }
}
