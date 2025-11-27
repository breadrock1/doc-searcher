use crate::domain::storage::models::{LargeDocument, LargeDocumentBuilder};

const LARGE_DOCUMENT_CONTENT: &str = "there is some huge content about current project";
const LARGE_DOCUMENT_CREATED_TIMESTAMP: i64 = 12375128745;

pub fn build_large_document() -> LargeDocument {
    LargeDocumentBuilder::default()
        .file_name("test-document.docx".to_string())
        .file_path("./test-document.docx".to_string())
        .file_size(1024)
        .created_at(LARGE_DOCUMENT_CREATED_TIMESTAMP)
        .modified_at(LARGE_DOCUMENT_CREATED_TIMESTAMP)
        .content(LARGE_DOCUMENT_CONTENT.to_string())
        .build()
        .unwrap()
}
