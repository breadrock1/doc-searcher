use mockall::mock;

use crate::domain::searcher::SearchError;
use crate::domain::searcher::models::{Pagination, PaginationParams, SearchingParams};
use crate::domain::searcher::{IPaginator, ISearcher};
use crate::domain::storage::StorageError;
use crate::domain::storage::models::{AllDocumentParts, DocumentPart};
use crate::domain::storage::models::{CreateIndexParams, StoredDocumentPartsInfo};
use crate::domain::storage::{IDocumentPartStorage, IIndexStorage};
use crate::shared::kernel::{DocumentPartId, IndexId, LargeDocumentId};

mock! {
    pub Storage{}

    impl Clone for Storage {
        fn clone(&self) -> Self;
    }

    #[async_trait::async_trait]
    impl IIndexStorage for Storage {
        async fn create_index(&self, index: &CreateIndexParams) -> Result<IndexId, StorageError>;
        async fn delete_index(&self, id: &IndexId) -> Result<(), StorageError>;
        async fn get_all_indexes(&self) -> Result<Vec<IndexId>, StorageError>;
        async fn get_index(&self, id: &IndexId) -> Result<IndexId, StorageError>;
    }

    #[async_trait::async_trait]
    impl IDocumentPartStorage for Storage{
        async fn store_document_parts(
            &self,
            index_id: &IndexId,
            all_doc_parts: AllDocumentParts,
        ) -> Result<StoredDocumentPartsInfo, StorageError>;

        async fn get_document_parts(
            &self,
            index_id: &IndexId,
            large_doc_id: &LargeDocumentId,
        ) -> Result<AllDocumentParts, StorageError>;

        async fn get_document_part(
            &self,
            index_id: &IndexId,
            doc_part_id: &DocumentPartId,
        ) -> Result<DocumentPart, StorageError>;

        async fn delete_document_parts(
            &self,
            index_id: &IndexId,
            large_doc_id: &LargeDocumentId,
        ) -> Result<(), StorageError>;
    }

    #[async_trait::async_trait]
    impl ISearcher for Storage {
        async fn search(&self, params: &SearchingParams) -> Result<Pagination, SearchError>;
    }

    #[async_trait::async_trait]
    impl IPaginator for Storage {
        async fn paginate(&self, params: &PaginationParams) -> Result<Pagination, SearchError>;
    }
}
