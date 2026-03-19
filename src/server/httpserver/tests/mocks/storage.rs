use mockall::mock;

use doc_search_core::domain::storage::models::{AllDocumentParts, DocumentPart};
use doc_search_core::domain::storage::models::{CreateIndexParams, StoredDocumentPartsInfo};
use doc_search_core::domain::storage::StorageError;
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use doc_search_core::shared::kernel::{DocumentPartId, IndexId, LargeDocumentId};

mock! {
    pub StorageService {}

    #[async_trait::async_trait]
    impl IIndexStorage for StorageService {
        async fn create_index(&self, index: &CreateIndexParams) -> Result<IndexId, StorageError>;
        async fn delete_index(&self, id: &IndexId) -> Result<(), StorageError>;
        async fn get_index(&self, id: &IndexId) -> Result<IndexId, StorageError>;
        async fn get_all_indexes(&self) -> Result<Vec<IndexId>, StorageError>;
    }

    #[async_trait::async_trait]
    impl IDocumentPartStorage for StorageService {
        async fn store_document_parts(
            &self,
            index: &IndexId,
            all_doc_parts: AllDocumentParts,
        ) -> Result<StoredDocumentPartsInfo, StorageError>;

        async fn get_document_parts(
            &self,
            index: &IndexId,
            large_doc_id: &LargeDocumentId,
        ) -> Result<AllDocumentParts, StorageError>;

        async fn get_document_part(
            &self,
            index: &IndexId,
            doc_part_id: &DocumentPartId,
        ) -> Result<DocumentPart, StorageError>;

        async fn delete_document_parts(
            &self,
            index: &IndexId,
            large_doc_id: &LargeDocumentId,
        ) -> Result<(), StorageError>;
    }
}
