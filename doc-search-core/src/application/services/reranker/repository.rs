use crate::application::services::reranker::RerankerResult;
use crate::domain::searcher::models::FoundedDocument;

/// Defines the interface for reranking.
///
/// # Methods
/// * `rerank` - Rerank content of search results.
#[async_trait::async_trait]
pub trait IReranker: Send + Sync {
    async fn rerank(
        &self,
        query: String,
        docs: Vec<FoundedDocument>,
    ) -> RerankerResult<Vec<FoundedDocument>>;
}
