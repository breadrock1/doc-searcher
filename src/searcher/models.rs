use derive_builder::Builder;
use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Deserialize, Serialize, Getters, MutGetters, ToSchema)]
pub struct Paginated<D: serde::Serialize> {
    #[getset(get = "pub", get_mut = "pub")]
    #[schema(value_type = Paginated<Vec<Document>>)]
    founded: D,
    #[schema(example = "10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}

impl<D: serde::Serialize> Paginated<D> {
    pub fn new(founded_docs: D) -> Self {
        Paginated {
            founded: founded_docs,
            scroll_id: None,
        }
    }

    pub fn new_with_id(founded_docs: D, id: String) -> Self {
        Paginated {
            founded: founded_docs,
            scroll_id: Some(id),
        }
    }

    pub fn new_with_opt_id(founded_docs: D, scroll: Option<String>) -> Self {
        Paginated {
            founded: founded_docs,
            scroll_id: scroll,
        }
    }

    pub fn scroll_id(&self) -> Option<String> {
        self.scroll_id.clone()
    }
}
