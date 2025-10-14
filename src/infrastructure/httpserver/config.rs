use gset::Getset;
use serde::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct HttpServerConfig {
    #[getset(get, vis = "pub")]
    address: String,
}
