use crate::application::services::embeddings::TokenizerResult;
use crate::shared::kernel::Tokens;

/// Defines the interface for embeddings computing.
///
/// # Methods
/// * `compute` - Computes embeddings for input text.
#[async_trait::async_trait]
pub trait ITokenizer: Send + Sync {
    async fn compute(&self, text: &str) -> TokenizerResult<Tokens>;
}
