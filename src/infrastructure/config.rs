use gset::Getset;
use serde_derive::Deserialize;

use crate::infrastructure::httpserver::HttpServerConfig;
use crate::infrastructure::osearch::OSearchConfig;
use crate::infrastructure::usermanager::UserManagerConfig;

#[cfg(feature = "enable-cache-redis")]
use crate::infrastructure::redis::config::RedisConfig;

#[derive(Clone, Deserialize, Getset)]
pub struct CacherConfig {
    #[getset(get, vis = "pub")]
    #[cfg(feature = "enable-cache-redis")]
    redis: RedisConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct ServerConfig {
    #[getset(get, vis = "pub")]
    http: HttpServerConfig,
    #[getset(get, vis = "pub")]
    usermanager: UserManagerConfig,
}

#[derive(Clone, Deserialize, Getset)]
pub struct StorageConfig {
    #[getset(get, vis = "pub")]
    opensearch: OSearchConfig,
}
