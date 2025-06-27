pub mod error;

use self::error::StorageResult;

use crate::application::dto::{
    Document, FullTextSearchParams, Index, PaginateParams, RetrieveDocumentParams,
    SemanticSearchParams, SemanticSearchWithTokensParams,
};
use crate::application::services::storage::error::PaginateResult;

#[async_trait::async_trait]
pub trait IndexManager {
    async fn create_index(&self, index: Index) -> StorageResult<Index>;
    async fn delete_index(&self, id: &str) -> StorageResult<()>;
    async fn get_all_indexes(&self) -> StorageResult<Vec<Index>>;
    async fn get_index(&self, id: &str) -> StorageResult<Index>;
}

#[async_trait::async_trait]
pub trait DocumentManager {
    async fn create_document(&self, index: &str, doc: Document) -> StorageResult<Document>;
    async fn get_document(&self, index: &str, id: &str) -> StorageResult<Document>;
    async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()>;
    async fn update_document(&self, index: &str, doc: Document) -> StorageResult<()>;
}

#[async_trait::async_trait]
pub trait DocumentSearcher {
    async fn retrieve(&self, params: &RetrieveDocumentParams) -> PaginateResult<Document>;
    async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<Document>;
    async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<Document>;
    async fn semantic_with_tokens(
        &self,
        params: &SemanticSearchWithTokensParams,
    ) -> PaginateResult<Document>;
}

#[async_trait::async_trait]
pub trait PaginateManager {
    async fn delete_session(&self, session_id: &str) -> StorageResult<()>;
    async fn paginate(&self, params: &PaginateParams) -> PaginateResult<Document>;
}
