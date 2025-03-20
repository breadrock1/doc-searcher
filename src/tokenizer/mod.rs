pub mod config;
pub mod baai;
pub mod errors;

use errors::TokenizerResult;

#[async_trait::async_trait]
pub trait TokenizerService {
    async fn compute(&self, text: &str) -> TokenizerResult<Vec<f64>>;
}
