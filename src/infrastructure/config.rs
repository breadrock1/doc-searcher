use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

use crate::infrastructure::osearch::config::OSearchConfig;
use crate::infrastructure::vectorizer::config::VectorizerConfig;

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
    baai: VectorizerConfig,
}

#[derive(Clone, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct StorageConfig {
    opensearch: OSearchConfig,
}
