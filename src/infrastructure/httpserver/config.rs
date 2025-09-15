use gset::Getset;
use serde::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct ServerConfig {
    #[getset(get, vis = "pub")]
    address: String,
}
