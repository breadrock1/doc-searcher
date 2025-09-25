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
    searcher: Arc<Searcher>,
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    pub fn new(searcher: Arc<Searcher>) -> Self {
        SearcherUseCase { searcher }
    }
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    #[tracing::instrument(skip(self), level = "info")]
    pub async fn retrieve(
        &self,
        ids: &str,
        params: &RetrieveDocumentParams,
    ) -> PaginateResult<FoundedDocument> {
        self.searcher.retrieve(ids, params).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<FoundedDocument> {
        self.searcher.fulltext(params).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<FoundedDocument> {
        self.searcher.semantic(params).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn hybrid(&self, params: &HybridSearchParams) -> PaginateResult<FoundedDocument> {
        self.searcher.hybrid(params).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn paginate(&self, params: &PaginateParams) -> PaginateResult<FoundedDocument> {
        self.searcher.paginate(params).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        self.searcher.delete_session(session_id).await
    }
}
