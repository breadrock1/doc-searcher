use derive_builder::Builder;
use gset::Getset;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Clone, Getset, Serialize, Deserialize)]
pub struct Index {
    #[getset(get, vis = "pub")]
    id: String,
    #[getset(get, vis = "pub")]
    name: String,
    #[getset(get, vis = "pub")]
    path: String,
}
