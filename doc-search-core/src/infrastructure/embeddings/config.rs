use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Getset)]
pub struct EmbeddingsConfig {
    #[getset(get, vis = "pub")]
    base_url: String,
    #[getset(get, vis = "pub")]
    api_key: String,
    #[getset(get_copy, vis = "pub")]
    timeout: u64,
    #[getset(get_copy, vis = "pub")]
    retries: u32,
    #[getset(get, vis = "pub")]
    model: String,
    #[getset(get_copy, vis = "pub")]
    overlap: f64,
    #[getset(get_copy, vis = "pub")]
    dimension: u32,
}
