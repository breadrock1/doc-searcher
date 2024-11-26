pub mod cacher;
pub mod config;
pub mod cors;
pub mod elastic;
pub mod embeddings;
pub mod errors;
pub mod logger;
pub mod metrics;
pub mod searcher;
pub mod storage;
pub mod swagger;

pub trait Connectable {
    type Config;
    type Error;
    type Service;

    fn connect(config: &Self::Config) -> Result<Self::Service, Self::Error>;
}
