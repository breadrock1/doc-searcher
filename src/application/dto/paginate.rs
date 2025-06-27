use derive_builder::Builder;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(unused_imports)]
use serde_json::json;

#[derive(Builder, Getters, Serialize, Deserialize, ToSchema)]
pub struct Paginated<D>
where
    D: serde::Serialize + Clone,
{
    #[schema(example = json!(vec![Document::example(None)]))]
    #[getset(get = "pub")]
    founded: D,
    #[schema(example = "dksfsjvJHZVFDskjdbfsdfsdfdsg")]
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
