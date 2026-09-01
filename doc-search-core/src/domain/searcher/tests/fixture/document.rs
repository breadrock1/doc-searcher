use rstest::fixture;

use crate::application::tests::fixture::LARGE_DOC_ID;
use crate::domain::searcher::models::{
    DocumentPartEntrails, DocumentPartEntrailsBuilder, FoundedDocument, FoundedDocumentBuilder,
};
use crate::domain::searcher::tests::fixture::*;
use crate::shared::kernel::LargeDocumentId;

#[fixture]
pub fn build_document_part_entrails() -> DocumentPartEntrails {
    DocumentPartEntrailsBuilder::default()
        .large_doc_id(LargeDocumentId(LARGE_DOC_ID.to_string()))
        .doc_part_id(DOC_PART_ID)
        .file_name(DOC_FILE_NAME.to_string())
        .file_path(DOC_FILE_PATH.to_string())
        .file_size(DOC_FILE_SIZE)
        .created_at(DOC_CREATED_AT)
        .modified_at(DOC_MODIFIED_AT)
        .content(Some(DOC_CONTENT.to_string()))
        .chunked_text(None)
        .embeddings(None)
        .metadata(None)
        .build()
        .expect("failed to build document part entrails")
}

pub fn build_founded_document(document_part: DocumentPartEntrails) -> FoundedDocument {
    FoundedDocumentBuilder::default()
        .id(DOC_ID.to_string())
        .index(INDEX.to_string())
        .score(Some(SCORE))
        .highlight(vec![HIGHLIGHT_ITEM.to_string()])
        .document(document_part)
        .build()
        .expect("failed to build founded document")
}
