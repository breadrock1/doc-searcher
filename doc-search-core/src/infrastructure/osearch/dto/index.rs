use gset::Getset;
use serde_derive::Deserialize;

use crate::shared::kernel::IndexId;

#[derive(Debug, Deserialize, Getset)]
pub struct IndexInformation {
    #[getset(get, vis = "pub")]
    index: String,
}

impl From<IndexInformation> for IndexId {
    fn from(value: IndexInformation) -> Self {
        IndexId(value.index)
    }
}
