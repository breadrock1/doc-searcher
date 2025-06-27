use derive_builder::Builder;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Getters, Serialize, Deserialize)]
pub struct Paginated<D>
where
    D: serde::Serialize + Clone,
{
    #[getset(get = "pub")]
    founded: D,
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}

impl<D> Paginated<D>
where
    D: serde::Serialize + Clone,
{
    pub fn builder() -> PaginatedBuilder<D> {
        PaginatedBuilder::default()
    }
}
