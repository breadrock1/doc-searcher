use doc_search_core::infrastructure::embeddings::EmbeddingsConfig;
use doc_search_core::infrastructure::osearch::OSearchConfig;
use gset::Getset;
use serde_derive::Deserialize;

use crate::server::httpserver::mw::cache::RedisConfig;
use crate::server::httpserver::HttpServerConfig;

#[derive(Clone, Deserialize, Getset)]
pub struct ServerConfig {
    #[getset(get, vis = "pub")]
    http: HttpServerConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct StorageConfig {
    #[getset(get, vis = "pub")]
    opensearch: OSearchConfig,
    #[getset(get, vis = "pub")]
    embeddings: EmbeddingsConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct CacheConfig {
    #[getset(get_copy, vis = "pub")]
    is_enabled: bool,
    #[getset(get, vis = "pub")]
    redis: RedisConfig,
}
