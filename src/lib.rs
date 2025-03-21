pub mod cacher;
pub mod config;
pub mod engine;
pub mod errors;
pub mod logger;
pub mod server;
pub mod tokenizer;

#[async_trait::async_trait]
pub trait ServiceConnect {
    type Config;
    type Error;
    type Client;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error>;
}
