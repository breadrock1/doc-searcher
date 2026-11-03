use mockall::mock;

use doc_search_core::application::services::reranker::IReranker;
use doc_search_core::application::services::reranker::RerankerResult;
use doc_search_core::domain::searcher::models::FoundedDocument;

mock! {
    pub RerankerProvider {}

    #[async_trait::async_trait]
    impl IReranker for RerankerProvider {
        async fn rerank(&self, query: String, docs: Vec<FoundedDocument>) -> RerankerResult<Vec<FoundedDocument>>;
    }
}
