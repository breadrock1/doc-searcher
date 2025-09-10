use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Getset)]
pub struct QdrantConfig {
    #[getset(get, vis = "pub")]
    address: String,
    #[getset(get, vis = "pub")]
    collection: String,
    #[getset(get_copy, vis = "pub")]
    dimension: u64,
}