use mockall::mock;

use crate::application::services::embeddings::ITokenizer;
use crate::application::services::embeddings::error::TokenizerResult;
use crate::shared::kernel::Tokens;

mock! {
    pub EmbeddingsProvider {}

    #[async_trait::async_trait]
    impl ITokenizer for EmbeddingsProvider {
        async fn compute(&self, chunks: String) -> TokenizerResult<Tokens>;
        async fn compute_bulk(&self, chunks: Vec<String>) -> TokenizerResult<Vec<Tokens>>;
    }
}
