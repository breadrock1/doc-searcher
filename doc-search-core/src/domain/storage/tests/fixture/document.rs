#![allow(dead_code)]

use rstest::fixture;

use crate::domain::storage::models::{
    DocumentPart, DocumentPartBuilder, LargeDocument, LargeDocumentBuilder,
};
use crate::shared::kernel::LargeDocumentId;

pub const LARGE_DOCUMENT_ID: &str = "098f6bcd4621d373cade4e832627b4f6";
pub const LARGE_DOCUMENT_CONTENT: &str = "there is some huge content about current project";
pub const LARGE_DOCUMENT_CREATED_TIMESTAMP: i64 = 12375128745;
pub const LARGE_DOCUMENT_FILE_NAME: &str = "test-document.docx";
pub const LARGE_DOCUMENT_FILE_PATH: &str = "./test-document.docx";
pub const LARGE_DOCUMENT_FILE_SIZE: u32 = 1024;

#[fixture]
pub fn build_large_document() -> LargeDocument {
    LargeDocumentBuilder::default()
        .file_name(LARGE_DOCUMENT_FILE_NAME.to_string())
        .file_path(LARGE_DOCUMENT_FILE_PATH.to_string())
        .file_size(LARGE_DOCUMENT_FILE_SIZE)
        .created_at(LARGE_DOCUMENT_CREATED_TIMESTAMP)
        .modified_at(LARGE_DOCUMENT_CREATED_TIMESTAMP)
        .content(LARGE_DOCUMENT_CONTENT.to_string())
        .metadata(None)
        .build()
        .expect("build large document fixture failed")
}

pub fn build_document_part(doc_part_id: usize) -> DocumentPart {
    DocumentPartBuilder::default()
        .large_doc_id(LargeDocumentId(LARGE_DOCUMENT_ID.to_string()))
        .doc_part_id(doc_part_id)
        .file_name(LARGE_DOCUMENT_FILE_NAME.to_string())
        .file_path(LARGE_DOCUMENT_FILE_PATH.to_string())
        .file_size(LARGE_DOCUMENT_FILE_SIZE)
        .created_at(LARGE_DOCUMENT_CREATED_TIMESTAMP)
        .modified_at(LARGE_DOCUMENT_CREATED_TIMESTAMP)
        .content(LARGE_DOCUMENT_CONTENT.to_string())
        .metadata(None)
        .build()
        .expect("build document part fixture failed")
}
