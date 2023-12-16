use crate::hasher::Hashed;
use thiserror::Error;

pub type HasherResult = Result<Hashed, HasherError>;

#[derive(Debug, Error)]
pub enum HasherError {
    #[error("Failed while generating hash from passed data")]
    FailedErr,
    #[error("Passed file path does not exist: {0}")]
    FileNotExist(String),
    #[error("Couldn't read a passed file: {0}")]
    ReadFileErr(String),
}
