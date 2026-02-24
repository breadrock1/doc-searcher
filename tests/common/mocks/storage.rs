use mockall::mock;

use doc_search_core::domain::storage::models::{
    AllDocumentParts, DocumentPart, StoredDocumentPartsInfo,
};
use doc_search_core::domain::storage::models::{CreateIndexParams, IndexId};
use doc_search_core::domain::storage::StorageError;
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};

mock! {
    pub StorageService {}

    #[async_trait::async_trait]
    impl IIndexStorage for StorageService {
        async fn create_index(&self, index: &CreateIndexParams) -> Result<IndexId, StorageError>;
        async fn delete_index(&self, id: &str) -> Result<(), StorageError>;
        async fn get_index(&self, id: &str) -> Result<IndexId, StorageError>;
        async fn get_all_indexes(&self) -> Result<Vec<IndexId>, StorageError>;
    }

    #[async_trait::async_trait]
    impl IDocumentPartStorage for StorageService {
        async fn store_document_parts(
            &self,
            index: &str,
            all_doc_parts: AllDocumentParts,
        ) -> Result<StoredDocumentPartsInfo, StorageError>;

        async fn get_document_parts(
            &self,
            index: &str,
            large_doc_id: &str,
        ) -> Result<AllDocumentParts, StorageError>;

        async fn get_document_part(
            &self,
            index: &str,
            doc_part_id: &str,
        ) -> Result<DocumentPart, StorageError>;

        async fn delete_document_parts(
            &self,
            index: &str,
            large_doc_id: &str,
        ) -> Result<(), StorageError>;
    }
}
