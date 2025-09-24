use std::sync::Arc;

use crate::application::services::storage::error::{PaginateResult, StorageResult};
use crate::application::services::storage::{DocumentSearcher, PaginateManager, StorageError};
use crate::application::services::tokenizer::{TokenizeError, TokenizeResult};
use crate::application::structures::params::{
    FullTextSearchParams, HybridSearchParams, PaginateParams, RetrieveDocumentParams,
    SemanticSearchParams,
};
use crate::application::structures::{FoundedDocument, InputContentBuilder, TokenizedContent};
use crate::application::usecase::TokenizerBoxed;

#[derive(Clone)]
pub struct SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    searcher: Arc<Searcher>,
    tokenizer: Arc<TokenizerBoxed>,
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    pub fn new(searcher: Arc<Searcher>, tokenizer: Arc<TokenizerBoxed>) -> Self {
        SearcherUseCase {
            searcher,
            tokenizer,
        }
    }
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn retrieve(
        &self,
        ids: &str,
        params: &RetrieveDocumentParams,
    ) -> PaginateResult<FoundedDocument> {
        self.searcher.retrieve(ids, params).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<FoundedDocument> {
        self.searcher.fulltext(params).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<FoundedDocument> {
        let _tokens = self
            .tokenize_query(params.query())
            .await
            .map_err(anyhow::Error::from)
            .map_err(StorageError::InternalError)?;

        // TODO: set tokens to params
        self.searcher.semantic(params).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn hybrid(&self, params: &HybridSearchParams) -> PaginateResult<FoundedDocument> {
        let _tokens = self
            .tokenize_query(params.query())
            .await
            .map_err(anyhow::Error::from)
            .map_err(StorageError::InternalError)?;

        // TODO: set tokens to params
        self.searcher.hybrid(params).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn paginate(&self, params: &PaginateParams) -> PaginateResult<FoundedDocument> {
        self.searcher.paginate(params).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        self.searcher.delete_session(session_id).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    async fn tokenize_query(&self, query: &str) -> TokenizeResult<TokenizedContent> {
        let input = InputContentBuilder::default()
            .content(query.to_string())
            .build()
            .map_err(TokenizeError::InputFormValidation)?;

        let tokenized_content = self.tokenizer.compute(&input).await?;
        Ok(tokenized_content)
    }
}
