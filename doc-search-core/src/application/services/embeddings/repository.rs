use crate::application::services::embeddings::TokenizerResult;
use crate::shared::kernel::Tokens;

/// Defines the interface for vllm computing.
///
/// # Methods
/// * `compute` - Computes vllm for input text.
#[async_trait::async_trait]
pub trait ITokenizer: Send + Sync {
    async fn compute(&self, chunks: String) -> TokenizerResult<Tokens>;
    async fn compute_bulk(&self, chunks: Vec<String>) -> TokenizerResult<Vec<Tokens>>;
}
