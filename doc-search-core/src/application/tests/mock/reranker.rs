use mockall::mock;

use crate::application::services::reranker::IReranker;
use crate::application::services::reranker::RerankerResult;
use crate::domain::searcher::models::FoundedDocument;

mock! {
    pub RerankerProvider {}

    #[async_trait::async_trait]
    impl IReranker for RerankerProvider {
        async fn rerank(&self, query: String, docs: Vec<FoundedDocument>) -> RerankerResult<Vec<FoundedDocument>>;
    }
}
