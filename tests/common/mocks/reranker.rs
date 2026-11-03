use doc_search_core::application::services::reranker::{IReranker, RerankerResult};
use doc_search_core::domain::searcher::models::FoundedDocument;

/// Reranker stub that leaves the incoming documents untouched, so integration
/// tests exercise the real search backend without an external reranker service.
#[derive(Clone, Default)]
pub struct StubReranker;

#[async_trait::async_trait]
impl IReranker for StubReranker {
    async fn rerank(
        &self,
        _: String,
        docs: Vec<FoundedDocument>,
    ) -> RerankerResult<Vec<FoundedDocument>> {
        Ok(docs)
    }
}
