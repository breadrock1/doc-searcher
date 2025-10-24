use mockall::mock;

use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::application::services::storage::StorageError;
use crate::application::structures::params::CreateIndexParams;
use crate::application::structures::params::{
    PaginateParams, RetrieveDocumentParams, FullTextSearchParams, HybridSearchParams,
    SemanticSearchParams
};
use crate::application::structures::{Document, Index, StoredDocument, FoundedDocument, Paginated};

mock! {
    pub Storage{}

    #[async_trait::async_trait]
    impl IndexManager for Storage {
        async fn create_index(&self, index: &CreateIndexParams) -> Result<String, StorageError>;
        async fn delete_index(&self, id: &str) -> Result<(), StorageError>;
        async fn get_all_indexes(&self) -> Result<Vec<Index>, StorageError>;
        async fn get_index(&self, id: &str) -> Result<Index, StorageError>;
    }


    #[async_trait::async_trait]
    impl DocumentManager for Storage{
        async fn store_document_parts(
            &self,
            index: &str,
            docs: &[Document],
        ) -> Result<Vec<StoredDocument>, StorageError>;
        async fn get_document(&self, index: &str, id: &str) -> Result<Document, StorageError>;
        async fn delete_document(&self, index: &str, id: &str) -> Result<(), StorageError>;
        async fn update_document(&self, index: &str, id: &str, doc: &Document) -> Result<(), StorageError>;
    }

    #[async_trait::async_trait]
    impl DocumentSearcher for Storage {
        async fn retrieve(
            &self,
            ids: &str,
            params: &RetrieveDocumentParams,
        ) -> Result<Paginated<Vec<FoundedDocument>>, StorageError>;
        async fn fulltext(&self, params: &FullTextSearchParams) -> Result<Paginated<Vec<FoundedDocument>>, StorageError>;
        async fn hybrid(&self, params: &HybridSearchParams) -> Result<Paginated<Vec<FoundedDocument>>, StorageError>;
        async fn semantic(&self, params: &SemanticSearchParams) -> Result<Paginated<Vec<FoundedDocument>>, StorageError>;
    }

    #[async_trait::async_trait]
    impl PaginateManager for Storage {
        async fn delete_session(&self, session_id: &str) -> Result<(), StorageError>;
        async fn paginate(&self, params: &PaginateParams) -> Result<Paginated<Vec<FoundedDocument>>, StorageError>;
    }
}
