use derive_builder::Builder;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Clone, Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct Index {
    id: String,
    name: String,
    path: String,
}
