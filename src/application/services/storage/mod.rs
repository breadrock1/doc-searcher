pub mod error;

pub use error::{PaginateResult, StorageError, StorageResult};

use crate::application::structures::params::CreateIndexParams;
use crate::application::structures::params::{
    FullTextSearchParams, HybridSearchParams, PaginateParams, RetrieveDocumentParams,
    SemanticSearchParams,
};
use crate::application::structures::{DocumentPart, FoundedDocument, Index, StoredDocumentPart};

#[async_trait::async_trait]
pub trait IndexManager {
    async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<String>;
    async fn delete_index(&self, id: &str) -> StorageResult<()>;
    async fn get_all_indexes(&self) -> StorageResult<Vec<Index>>;
    async fn get_index(&self, id: &str) -> StorageResult<Index>;
}

#[async_trait::async_trait]
pub trait DocumentManager {
    async fn store_document_parts(
        &self,
        index: &str,
        docs: &[DocumentPart],
    ) -> StorageResult<Vec<StoredDocumentPart>>;
    async fn get_document(&self, index: &str, id: &str) -> StorageResult<DocumentPart>;
    async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()>;
    async fn update_document(&self, index: &str, id: &str, doc: &DocumentPart)
        -> StorageResult<()>;
}

#[async_trait::async_trait]
pub trait DocumentSearcher {
    async fn retrieve(
        &self,
        ids: &str,
        params: &RetrieveDocumentParams,
    ) -> PaginateResult<FoundedDocument>;
    async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<FoundedDocument>;
    async fn hybrid(&self, params: &HybridSearchParams) -> PaginateResult<FoundedDocument>;
    async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<FoundedDocument>;
}

#[async_trait::async_trait]
pub trait PaginateManager {
    async fn delete_session(&self, session_id: &str) -> StorageResult<()>;
    async fn paginate(&self, params: &PaginateParams) -> PaginateResult<FoundedDocument>;
}
