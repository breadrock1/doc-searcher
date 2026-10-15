use doc_search_core::application::services::embeddings::ITokenizer;
use doc_search_core::application::services::embeddings::TokenizerResult;
use doc_search_core::shared::kernel::Tokens;

#[derive(Clone, Default)]
pub struct DummyEmbeddingsProvider;

#[async_trait::async_trait]
impl ITokenizer for DummyEmbeddingsProvider {
    async fn compute(&self, _text: &str) -> TokenizerResult<Tokens> {
        Ok(vec![0.1, 0.2, 0.3].into())
    }
}
