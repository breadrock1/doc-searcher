use crate::application::structures::{Index, IndexBuilder};
use derive_builder::Builder;
use gset::Getset;

#[derive(Builder, Getset)]
pub struct Resource {
    #[getset(get, vis = "pub")]
    id: String,
    #[getset(get, vis = "pub")]
    name: String,
    #[getset(get, vis = "pub")]
    created_at: chrono::NaiveDateTime,
    #[getset(get_copy, vis = "pub")]
    is_public: bool,
}

impl From<Resource> for Index {
    fn from(resource: Resource) -> Self {
        IndexBuilder::default()
            .id(resource.id)
            .name(resource.name)
            .path("".to_string())
            .build()
            .unwrap()
    }
}
