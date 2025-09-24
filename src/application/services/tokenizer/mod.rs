mod errors;

pub use errors::{TokenizeError, TokenizeResult};

use crate::application::structures::{TokenizedContent, InputContent};

#[async_trait::async_trait]
pub trait TokenizeProvider {
    async fn compute(&self, form: &InputContent) -> TokenizeResult<TokenizedContent>;
}
