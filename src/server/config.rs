use doc_search_core::infrastructure::osearch::OSearchConfig;
use gset::Getset;
use serde_derive::Deserialize;

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
}
