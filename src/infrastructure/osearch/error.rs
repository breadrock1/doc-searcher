use crate::application::services::storage::error::StorageError;

impl From<opensearch::Error> for StorageError {
    fn from(err: opensearch::Error) -> Self {
        StorageError::ServiceError(anyhow::Error::from(err))
    }
}
