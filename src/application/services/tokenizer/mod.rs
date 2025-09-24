mod errors;

pub use errors::{TokenizeError, TokenizeResult};

use crate::application::structures::{InputContent, TokenizedContent};

#[async_trait::async_trait]
pub trait TokenizeProvider {
    async fn compute(&self, form: &InputContent) -> TokenizeResult<TokenizedContent>;
}
