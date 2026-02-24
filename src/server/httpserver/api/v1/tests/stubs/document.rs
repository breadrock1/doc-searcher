use serde_json::json;
use serde_json::Value;

use doc_search_core::domain::storage::models::{
    DocumentPart, DocumentPartBuilder, StoredDocumentPartsInfo,
};

use crate::server::httpserver::api::v1::schema::DocumentPartSchema;

use super::constants::{
    DOCUMENT_CONTENT, DOCUMENT_CREATED_AT, DOCUMENT_FILE_NAME, DOCUMENT_FILE_PATH,
    DOCUMENT_FILE_SIZE, DOCUMENT_MODIFIED_AT, FIRST_DOC_PART_ID, LARGE_DOCUMENT_ID,
};

pub fn stored_document_info_json_object() -> Value {
    json!({
        "large_doc_id": LARGE_DOCUMENT_ID,
        "first_part_id": FIRST_DOC_PART_ID,
        "doc_parts_amount": 2,
    })
}

pub fn stored_documents_info_json_object() -> Value {
    json!([stored_document_info_json_object()])
}

pub fn create_document_json_object() -> Value {
    json!({
        "file_name": DOCUMENT_FILE_NAME,
        "file_path": DOCUMENT_FILE_PATH,
        "content": DOCUMENT_CONTENT,
        "file_size": DOCUMENT_FILE_SIZE,
        "created_at": DOCUMENT_CREATED_AT,
        "modified_at": DOCUMENT_MODIFIED_AT,
    })
}

pub fn create_documents_json_object() -> Value {
    json!([create_document_json_object()])
}

pub fn build_document_part_json_object(doc_part_id: usize) -> Value {
    let document_part = build_document_part(doc_part_id);
    let document_part_schema = DocumentPartSchema::try_from(document_part)
        .expect("failed to convert document part to schema");

    serde_json::to_value(document_part_schema)
        .expect("failed to convert document part schema to JSON")
}

pub fn document_parts_json_object() -> Value {
    json!([
        build_document_part_json_object(1),
        build_document_part_json_object(2)
    ])
}

pub fn stored_document_info() -> StoredDocumentPartsInfo {
    StoredDocumentPartsInfo {
        large_doc_id: LARGE_DOCUMENT_ID.to_string(),
        first_part_id: FIRST_DOC_PART_ID.to_string(),
        doc_parts_amount: 2,
    }
}

pub fn build_document_part(doc_part_id: usize) -> DocumentPart {
    DocumentPartBuilder::default()
        .large_doc_id(LARGE_DOCUMENT_ID.to_string())
        .doc_part_id(doc_part_id)
        .file_name(DOCUMENT_FILE_NAME.to_string())
        .file_path(DOCUMENT_FILE_PATH.to_string())
        .file_size(DOCUMENT_FILE_SIZE)
        .created_at(DOCUMENT_CREATED_AT)
        .modified_at(DOCUMENT_MODIFIED_AT)
        .content(DOCUMENT_CONTENT.to_string())
        .metadata(None)
        .build()
        .expect("build document part fixture failed")
}
