use crate::domain::support::storage::params::CreateIndexParams;
use crate::domain::support::storage::{DocumentPart, Index, StorageResult};

#[async_trait::async_trait]
pub trait IIndexStorage {
    async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<String>;
    async fn delete_index(&self, id: &str) -> StorageResult<()>;
    async fn get_all_indexes(&self) -> StorageResult<Vec<Index>>;
    async fn get_index(&self, id: &str) -> StorageResult<Index>;
}

#[async_trait::async_trait]
pub trait IDocumentStorage {
    async fn get_document(&self, index: &str, id: &str) -> StorageResult<DocumentPart>;
    async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()>;
    async fn update_document(&self, index: &str, id: &str, doc: &DocumentPart) -> StorageResult<()>;
    async fn store_document_parts(&self, index: &str, docs: &[DocumentPart]) -> StorageResult<Vec<String>>;
}
