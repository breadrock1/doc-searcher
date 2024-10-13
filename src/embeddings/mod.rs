use crate::errors::WebResult;

pub mod config;
pub mod native;

#[async_trait::async_trait]
pub trait EmbeddingsService {
    async fn load_from_text(&self, text: &str) -> WebResult<Vec<f64>>;
}
