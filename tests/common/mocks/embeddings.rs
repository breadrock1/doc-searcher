use doc_search_core::application::services::embeddings::{ITokenizer, TokenizerResult};
use doc_search_core::shared::kernel::Tokens;

#[derive(Clone, Default)]
pub struct StubEmbeddingsProvider;

#[async_trait::async_trait]
impl ITokenizer for StubEmbeddingsProvider {
    async fn compute(&self, _: String) -> TokenizerResult<Tokens> {
        Ok(Vec::new().into())
    }

    async fn compute_bulk(&self, _: Vec<String>) -> TokenizerResult<Vec<Tokens>> {
        Ok(Vec::new())
    }
}
