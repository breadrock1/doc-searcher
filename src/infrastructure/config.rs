use gset::Getset;
use serde_derive::Deserialize;

use crate::infrastructure::osearch::config::OSearchConfig;

#[cfg(feature = "enable-cache-redis")]
use crate::infrastructure::redis::config::RedisConfig;

#[derive(Clone, Deserialize, Getset)]
pub struct CacherConfig {
    #[getset(get, vis = "pub")]
    #[cfg(feature = "enable-cache-redis")]
    redis: RedisConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct StorageConfig {
    #[getset(get, vis = "pub")]
    opensearch: OSearchConfig,
}
