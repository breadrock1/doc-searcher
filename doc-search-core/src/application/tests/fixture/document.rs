use rstest::fixture;

use crate::domain::storage::models::LargeDocument;
use crate::domain::storage::models::document::LargeDocumentBuilder;

pub const LARGE_DOC_ID: &str = "29346839246dsf987a1173sfa7sd781h";
pub const FIRST_DOC_PART_ID: &str = "k3j5b49246dsf987a1173sfa7sd781h";
pub const DOC_FILE_NAME: &str = "test-document.docx";
pub const DOC_FILE_PATH: &str = "./test-document.docx";
pub const DOC_FILE_SIZE: u32 = 1024;
pub const DOC_FILE_TIMESTAMP: i64 = 1750957215;
pub const DOC_FILE_LARGE_CONTENT: &str = include_str!("../resources/doc-content-large.txt");
pub const DOC_FILE_SHORT_CONTENT: &str = include_str!("../resources/doc-content-short.txt");

#[fixture]
pub fn build_large_document() -> LargeDocument {
    LargeDocumentBuilder::default()
        .file_name(DOC_FILE_NAME.to_string())
        .file_path(DOC_FILE_PATH.to_string())
        .file_size(DOC_FILE_SIZE)
        .created_at(DOC_FILE_TIMESTAMP)
        .modified_at(DOC_FILE_TIMESTAMP)
        .content(DOC_FILE_LARGE_CONTENT.to_string())
        .build()
        .unwrap()
}

#[fixture]
pub fn build_short_document() -> LargeDocument {
    LargeDocumentBuilder::default()
        .file_name(DOC_FILE_NAME.to_string())
        .file_path(DOC_FILE_PATH.to_string())
        .file_size(DOC_FILE_SIZE)
        .created_at(DOC_FILE_TIMESTAMP)
        .modified_at(DOC_FILE_TIMESTAMP)
        .content(DOC_FILE_SHORT_CONTENT.to_string())
        .build()
        .unwrap()
}
