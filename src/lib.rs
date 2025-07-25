pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod logger;

#[async_trait::async_trait]
pub trait ServiceConnect {
    type Config;
    type Client;
    type Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error>;
}
