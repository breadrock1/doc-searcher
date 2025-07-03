use serde_derive::Deserialize;

use crate::application::dto::{Document, Index, FoundedDocument};

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

#[derive(Deserialize)]
pub struct SourceDocument {
    _id: String,
    _source: Document,
    highlight: Option<HighlightContent>,
}

impl From<SourceDocument> for FoundedDocument {
    fn from(src_doc: SourceDocument) -> Self {
        let highlight = src_doc
            .highlight
            .map(|it| it.content)
            .unwrap_or_default();

        FoundedDocument::builder()
            .document(src_doc._source)
            .highlight(highlight)
            .build()
            .unwrap()
    }
}

impl From<SourceDocument> for Document {
    fn from(src_doc: SourceDocument) -> Self {
        src_doc._source
    }
}

#[derive(Deserialize)]
struct HighlightContent {
    content: Vec<String>,
}
