use getset::Getters;
use serde_derive::Deserialize;

use crate::infrastructure::osearch::config::OSearchConfig;

#[cfg(feature = "enable-cache-redis")]
use crate::infrastructure::redis::config::RedisConfig;

#[derive(Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct CacherConfig {
    #[cfg(feature = "enable-cache-redis")]
    redis: RedisConfig,
}

#[derive(Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct StorageConfig {
    opensearch: OSearchConfig,
}
