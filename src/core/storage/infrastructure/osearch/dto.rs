use gset::Getset;
use serde_derive::Deserialize;

use crate::core::storage::domain::{DocumentPart, Index, IndexBuilder};

#[derive(Debug, Deserialize, Getset)]
pub struct OSearchIndex {
    #[getset(get, vis = "pub")]
    index: String,
}

impl From<&OSearchIndex> for Index {
    fn from(value: &OSearchIndex) -> Self {
        IndexBuilder::default()
            .id(value.index.to_owned())
            .name(value.index.to_owned())
            .path("./".to_owned())
            .build()
            .unwrap()
    }
}

#[derive(Deserialize)]
pub struct SourceDocument {
    _id: String,
    _index: String,
    _source: DocumentPart,
}

impl From<SourceDocument> for DocumentPart {
    fn from(src_doc: SourceDocument) -> Self {
        src_doc._source
    }
}
