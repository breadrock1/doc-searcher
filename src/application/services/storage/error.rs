use thiserror::Error;

use crate::application::dto::Paginated;
use crate::application::services::tokenizer::TokenizerError;

pub type StorageResult<T> = Result<T, StorageError>;
pub type PaginateResult<T> = StorageResult<Paginated<Vec<T>>>;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(anyhow::Error),
    #[error("request timeout: {0}")]
    RequestTimeout(anyhow::Error),
    #[error("target object haven't been founded: {0}")]
    NotFound(anyhow::Error),
    #[error("returned error into response: {0}")]
    ServiceError(anyhow::Error),
    #[error("failed to deserialize response data: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("returned error into response: {0}")]
    RuntimeError(String),

    #[error("failed to compute tokens: {0}")]
    ComputeEmbeddings(#[from] TokenizerError),
    #[error("index has not been founded: {0}")]
    IndexNotFound(anyhow::Error),
}
