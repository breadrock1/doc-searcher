use std::sync::Arc;

use crate::application::dto::{
    Document, FullTextSearchParams, PaginateParams, RetrieveDocumentParams, SemanticSearchParams,
    SemanticSearchWithTokensParams,
};
use crate::application::services::storage::error::{PaginateResult, StorageResult};
use crate::application::services::storage::{DocumentSearcher, PaginateManager};
use crate::application::services::tokenizer::Tokenizer;

#[derive(Clone)]
pub struct SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    client: Arc<Searcher>,
    tokenizer: Option<Arc<Box<dyn Tokenizer + Send + Sync>>>,
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    pub fn new(
        client: Arc<Searcher>,
        tokenizer: Option<Arc<Box<dyn Tokenizer + Send + Sync>>>,
    ) -> Self {
        SearcherUseCase { client, tokenizer }
    }
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
{
    pub async fn retrieve(&self, params: &RetrieveDocumentParams) -> PaginateResult<Document> {
        self.client.retrieve(params).await
    }

    pub async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<Document> {
        self.client.fulltext(params).await
    }

    pub async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<Document> {
        if let Some(tokenizer) = self.tokenizer.clone() {
            let tokens = tokenizer.compute(params.query()).await?;
            let params = SemanticSearchWithTokensParams::build_from_semantic_params(params, tokens);
            return self.client.semantic_with_tokens(&params).await;
        }

        self.client.semantic(params).await
    }

    pub async fn paginate(&self, params: &PaginateParams) -> PaginateResult<Document> {
        self.client.paginate(params).await
    }

    pub async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        self.client.delete_session(session_id).await
    }
}
