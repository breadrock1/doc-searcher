use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Getters, CopyGetters, Serialize, Deserialize, ToSchema)]
pub struct Index {
    #[schema(example = "test-folder")]
    #[getset(get = "pub")]
    id: String,
    #[schema(example = "Test Folder")]
    #[getset(get = "pub")]
    name: String,
    #[schema(example = "./")]
    #[getset(get = "pub")]
    path: String,
}

impl Index {
    pub fn builder() -> IndexBuilder {
        IndexBuilder::default()
    }
}
