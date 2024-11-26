pub mod config;
pub mod errors;
pub mod native;

#[async_trait::async_trait]
pub trait EmbeddingsService {
    async fn load_from_text(&self, text: &str) -> errors::EmbeddingsResult<Vec<f64>>;
}
