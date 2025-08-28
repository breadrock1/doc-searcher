use std::sync::Arc;

use crate::application::services::storage::error::{PaginateResult, StorageResult};
use crate::application::services::storage::{DocumentSearcher, PaginateManager};
use crate::application::structures::params::{
    FullTextSearchParams, HybridSearchParams, PaginateParams, RetrieveDocumentParams,
    SemanticSearchParams,
};
use crate::application::structures::FoundedDocument;

#[derive(Clone)]
pub struct SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    client: Arc<Searcher>,
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    pub fn new(client: Arc<Searcher>) -> Self {
        SearcherUseCase { client }
    }
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    pub async fn retrieve(
        &self,
        ids: &str,
        params: &RetrieveDocumentParams,
    ) -> PaginateResult<FoundedDocument> {
        self.client.retrieve(ids, params).await
    }

    pub async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<FoundedDocument> {
        self.client.fulltext(params).await
    }

    pub async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<FoundedDocument> {
        self.client.semantic(params).await
    }

    pub async fn hybrid(&self, params: &HybridSearchParams) -> PaginateResult<FoundedDocument> {
        self.client.hybrid(params).await
    }

    pub async fn paginate(&self, params: &PaginateParams) -> PaginateResult<FoundedDocument> {
        self.client.paginate(params).await
    }

    pub async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        self.client.delete_session(session_id).await
    }
}
