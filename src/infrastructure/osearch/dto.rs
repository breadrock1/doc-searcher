use crate::application::dto::Index;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OSearchIndex {
    index: String,
}

impl From<&OSearchIndex> for Index {
    fn from(value: &OSearchIndex) -> Self {
        Index::builder()
            .id(value.index.to_owned())
            .name(value.index.to_owned())
            .path("./".to_owned())
            .build()
            .unwrap()
    }
}
