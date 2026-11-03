use mockall::mock;

use doc_search_core::application::services::embeddings::error::TokenizerResult;
use doc_search_core::application::services::embeddings::ITokenizer;
use doc_search_core::shared::kernel::Tokens;

mock! {
    pub EmbeddingsProvider {}

    #[async_trait::async_trait]
    impl ITokenizer for EmbeddingsProvider {
        async fn compute(&self, chunks: String) -> TokenizerResult<Tokens>;
        async fn compute_bulk(&self, chunks: Vec<String>) -> TokenizerResult<Vec<Tokens>>;
    }
}
