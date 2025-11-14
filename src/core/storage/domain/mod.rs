mod error;
pub use error::{StorageError, StorageResult};

mod model;
pub use model::{DocumentPart, DocumentPartBuilder, DocumentPartBuilderError};
pub use model::{Embeddings, StoredDocumentPart};
pub use model::{Index, IndexBuilder, IndexBuilderError};

mod params;
pub use params::{CreateIndexParams, CreateIndexParamsBuilder, CreateIndexParamsBuilderError};
pub use params::{KnnIndexParams, KnnIndexParamsBuilder, KnnIndexParamsBuilderError};

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

