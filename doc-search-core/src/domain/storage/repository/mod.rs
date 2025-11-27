use crate::domain::storage::StorageResult;
use crate::domain::storage::models::{AllDocumentParts, DocumentPart, StoredDocumentPartsInfo};
use crate::domain::storage::models::{CreateIndexParams, IndexId};

#[async_trait::async_trait]
pub trait IIndexStorage {
    async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<IndexId>;
    async fn delete_index(&self, id: &str) -> StorageResult<()>;
    async fn get_index(&self, id: &str) -> StorageResult<IndexId>;
    async fn get_all_indexes(&self) -> StorageResult<Vec<IndexId>>;
}

#[async_trait::async_trait]
pub trait IDocumentPartStorage {
    async fn store_document_parts(
        &self,
        index: &str,
        all_doc_parts: AllDocumentParts,
    ) -> StorageResult<StoredDocumentPartsInfo>;

    async fn get_document_parts(
        &self,
        index: &str,
        large_doc_id: &str,
    ) -> StorageResult<AllDocumentParts>;

    async fn get_document_part(
        &self,
        index: &str,
        doc_part_id: &str,
    ) -> StorageResult<DocumentPart>;
    async fn delete_document_parts(&self, index: &str, large_doc_id: &str) -> StorageResult<()>;
}
