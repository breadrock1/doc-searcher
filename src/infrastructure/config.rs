use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

use crate::infrastructure::baii::config::BAAIConfig;
use crate::infrastructure::osearch::config::OSearchConfig;

#[cfg(feature = "enable-cache-redis")]
use crate::infrastructure::redis::config::RedisConfig;

#[derive(Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct CacherConfig {
    #[cfg(feature = "enable-cache-redis")]
    redis: RedisConfig,
}

#[derive(Clone, Deserialize, Getters, CopyGetters)]
pub struct TokenizerConfig {
    #[getset(get_copy = "pub")]
    enable: bool,
    #[getset(get = "pub")]
    baai: BAAIConfig,
}

#[derive(Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct StorageConfig {
    opensearch: OSearchConfig,
}
