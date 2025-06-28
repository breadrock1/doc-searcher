mod error;

pub use error::{TokenizerError, TokenizerResult};

use crate::application::dto::Tokens;

#[async_trait::async_trait]
pub trait Tokenizer {
    async fn compute(&self, text: &str) -> TokenizerResult<Tokens>;
}
