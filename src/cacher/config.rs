use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
#[getset(get = "pub")]
pub struct CacherConfig {
    #[cfg(feature = "enable-cacher")]
    redis: RedisConfig,
}
