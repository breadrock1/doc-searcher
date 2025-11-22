#[cfg(test)]
pub(crate) mod tests;

pub mod models;

mod repository;
pub use repository::{IDocumentPartStorage, IIndexStorage};

mod error;
pub use error::{StorageError, StorageResult};
