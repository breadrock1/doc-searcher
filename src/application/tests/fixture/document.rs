use rstest::fixture;

use crate::application::structures::{DocumentPart, DocumentPartBuilder};

pub const DOC_ID: &str = "29346839246dsf987a1173sfa7sd781h";
pub const DOC_FILE_NAME: &str = "test-document.docx";
pub const DOC_FILE_PATH: &str = "./test-document.docx";
pub const DOC_FILE_SIZE: u32 = 1024;
pub const DOC_FILE_TIMESTAMP: i64 = 1750957215;
pub const DOC_FILE_LARGE_CONTENT: &str = include_str!("../resources/doc-content-large.txt");
pub const DOC_FILE_SHORT_CONTENT: &str = include_str!("../resources/doc-content-short.txt");

#[fixture]
pub fn build_large_document() -> DocumentPart {
    DocumentPartBuilder::default()
        .file_name(DOC_FILE_NAME.to_string())
        .file_path(DOC_FILE_PATH.to_string())
        .file_size(DOC_FILE_SIZE)
        .created_at(DOC_FILE_TIMESTAMP)
        .modified_at(DOC_FILE_TIMESTAMP)
        .content(Some(DOC_FILE_LARGE_CONTENT.to_string()))
        .chunked_text(None)
        .embeddings(None)
        .doc_part_id(0)
        .build()
        .unwrap()
}

#[fixture]
pub fn build_short_document() -> DocumentPart {
    DocumentPartBuilder::default()
        .file_name(DOC_FILE_NAME.to_string())
        .file_path(DOC_FILE_PATH.to_string())
        .file_size(DOC_FILE_SIZE)
        .created_at(DOC_FILE_TIMESTAMP)
        .modified_at(DOC_FILE_TIMESTAMP)
        .content(Some(DOC_FILE_SHORT_CONTENT.to_string()))
        .chunked_text(None)
        .embeddings(None)
        .doc_part_id(0)
        .build()
        .unwrap()
}
