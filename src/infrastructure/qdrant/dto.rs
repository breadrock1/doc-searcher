use std::collections::HashMap;
use gset::Getset;
use qdrant_client::qdrant::Value;
use serde_derive::Deserialize;

use crate::application::structures::{Document, DocumentBuilder, FoundedDocument, FoundedDocumentBuilder};

#[derive(Deserialize, Getset)]
pub struct SourceDocument {
    _id: String,
    _index: String,
    _source: Document,
    #[getset(set, vis = "pub")]
    _score: Option<f64>,
    highlight: Option<HighlightContent>,
}

#[derive(Deserialize)]
struct HighlightContent {
    content: Vec<String>,
}


impl From<HashMap<String, Value>> for SourceDocument {
    fn from(value: HashMap<String, Value>) -> Self {
        SourceDocument {
            _id: value.get("id").unwrap().to_string(),
            _index: value.get("index").unwrap().to_string(),
            _source: DocumentBuilder::default().build().unwrap(),
            _score: None,
            highlight: None,
        }
        // DocumentBuilder::default()
        //     .build()
        //     .unwrap()
    }
}

impl From<SourceDocument> for FoundedDocument {
    fn from(src_doc: SourceDocument) -> Self {
        let highlight = src_doc.highlight.map(|it| it.content).unwrap_or_default();

        FoundedDocumentBuilder::default()
            .id(src_doc._id)
            .folder_id(src_doc._index)
            .document(src_doc._source)
            .highlight(highlight)
            .score(src_doc._score)
            .build()
            .unwrap()
    }
}
