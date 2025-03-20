use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

#[cfg(feature = "enable-cacher-redis")]
use crate::cacher::redis::config::RedisConfig;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
#[getset(get = "pub")]
pub struct CacherConfig {
    #[cfg(feature = "enable-cacher-redis")]
    redis: RedisConfig,
}
