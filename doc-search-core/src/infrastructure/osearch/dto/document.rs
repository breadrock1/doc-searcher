use anyhow::Context;
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

use crate::domain::searcher::models::{DocumentPartEntrails, DocumentPartEntrailsBuilder};
use crate::domain::storage::models::DocumentPart;
use crate::infrastructure::osearch::dto::metadata::SourceDocumentMetadata;
use crate::infrastructure::osearch::error::OSearchError;

#[derive(Builder, Deserialize, Serialize)]
pub struct SourceDocument {
    pub large_doc_id: String,
    pub doc_part_id: usize,
    pub file_name: String,
    pub file_path: String,
    pub file_size: u32,
    pub created_at: i64,
    pub modified_at: i64,
    pub content: Option<String>,
    pub metadata: Option<SourceDocumentMetadata>,
}

impl TryFrom<SourceDocument> for DocumentPartEntrails {
    type Error = OSearchError;

    fn try_from(src_doc: SourceDocument) -> Result<Self, Self::Error> {
        let metadata = match src_doc.metadata {
            Some(meta) => meta.try_into().ok(),
            None => None,
        };

        DocumentPartEntrailsBuilder::default()
            .large_doc_id(src_doc.large_doc_id)
            .doc_part_id(src_doc.doc_part_id)
            .file_name(src_doc.file_name)
            .file_path(src_doc.file_path)
            .file_size(src_doc.file_size)
            .created_at(src_doc.created_at)
            .modified_at(src_doc.modified_at)
            .content(src_doc.content)
            .chunked_text(None)
            .embeddings(None)
            .metadata(metadata)
            .build()
            .context("failed to build document part entrails")
            .map_err(OSearchError::ValidationError)
    }
}

impl TryFrom<DocumentPart> for SourceDocument {
    type Error = OSearchError;

    fn try_from(doc_part: DocumentPart) -> Result<Self, Self::Error> {
        let metadata = match doc_part.metadata {
            Some(meta) => meta.try_into().ok(),
            None => None,
        };

        SourceDocumentBuilder::default()
            .large_doc_id(doc_part.large_doc_id)
            .doc_part_id(doc_part.doc_part_id)
            .file_name(doc_part.file_name)
            .file_path(doc_part.file_path)
            .file_size(doc_part.file_size)
            .created_at(doc_part.created_at)
            .modified_at(doc_part.modified_at)
            .content(Some(doc_part.content))
            .metadata(metadata)
            .build()
            .context("failed to build document part")
            .map_err(OSearchError::ValidationError)
    }
}
