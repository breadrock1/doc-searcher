use derive_builder::Builder;
use gset::Getset;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Getset, Serialize, Deserialize)]
pub struct Paginated<D>
where
    D: serde::Serialize + Clone,
{
    #[getset(get, vis = "pub")]
    founded: D,
    #[getset(get, vis = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}
