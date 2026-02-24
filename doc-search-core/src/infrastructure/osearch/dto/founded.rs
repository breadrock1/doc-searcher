use anyhow::Context;
use serde_derive::Deserialize;

use crate::domain::searcher::models::{FoundedDocument, FoundedDocumentBuilder};
use crate::domain::storage::models::{DocumentPart, DocumentPartBuilder};
use crate::infrastructure::osearch::dto::document::SourceDocument;
use crate::infrastructure::osearch::error::OSearchError;

#[derive(Deserialize)]
pub struct FoundedDocumentInfo {
    _id: String,
    _index: String,
    _score: Option<f64>,
    _source: SourceDocument,
    highlight: Option<HighlightContent>,
}

#[derive(Deserialize)]
struct HighlightContent {
    content: Vec<String>,
}

impl TryFrom<FoundedDocumentInfo> for FoundedDocument {
    type Error = OSearchError;

    fn try_from(doc_info: FoundedDocumentInfo) -> Result<Self, Self::Error> {
        let highlight = doc_info.highlight.map(|it| it.content).unwrap_or_default();
        let document = doc_info._source.try_into()?;
        FoundedDocumentBuilder::default()
            .id(doc_info._id)
            .index(doc_info._index)
            .document(document)
            .highlight(highlight)
            .score(doc_info._score)
            .build()
            .context("failed to build founded document")
            .map_err(OSearchError::ValidationError)
    }
}

impl TryFrom<FoundedDocumentInfo> for DocumentPart {
    type Error = OSearchError;

    fn try_from(doc_info: FoundedDocumentInfo) -> Result<Self, Self::Error> {
        let src_doc = doc_info._source;
        let metadata = match src_doc.metadata {
            Some(meta) => meta.try_into().ok(),
            None => None,
        };

        DocumentPartBuilder::default()
            .large_doc_id(src_doc.large_doc_id)
            .doc_part_id(src_doc.doc_part_id)
            .file_name(src_doc.file_name)
            .file_path(src_doc.file_path)
            .file_size(src_doc.file_size)
            .created_at(src_doc.created_at)
            .modified_at(src_doc.modified_at)
            .content(src_doc.content.unwrap_or_default())
            .metadata(metadata)
            .build()
            .context("failed to build founded document")
            .map_err(OSearchError::ValidationError)
    }
}
