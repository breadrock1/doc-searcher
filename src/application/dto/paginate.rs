use derive_builder::Builder;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct Paginated<D>
where
    D: serde::Serialize + Clone,
{
    founded: D,
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}
