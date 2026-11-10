use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Getset)]
pub struct ResourceManagerConfig {
    #[getset(get, vis = "pub")]
    base_url: String,
    #[getset(get_copy, vis = "pub")]
    timeout: u64,
}
