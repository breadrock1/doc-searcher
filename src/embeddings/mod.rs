pub mod config;
pub mod native;

#[allow(async_fn_in_trait)]
pub trait EmbeddingsLoader {
    type Error;

    async fn load_from_text(&self, text: &str) -> Result<Vec<f64>, Self::Error>;
}
