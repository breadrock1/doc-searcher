use crate::shared::kernel::{DocumentPartId, IndexId, LargeDocumentId};

const LARGE_DOC_ID_VALUE: &str = "large-doc-1";
const DOC_PART_ID_VALUE: &str = "doc-part-1";
const INDEX_ID_VALUE: &str = "index-1";

#[test]
fn test_large_document_id_deref_display_and_as_string() {
    let id = LargeDocumentId(LARGE_DOC_ID_VALUE.to_string());

    assert_eq!(LARGE_DOC_ID_VALUE, &*id);
    assert_eq!(11, id.len());
    assert_eq!(LARGE_DOC_ID_VALUE, id.as_string());
    assert_eq!(LARGE_DOC_ID_VALUE, format!("{id}"));
}

#[test]
fn test_document_part_id_deref_and_as_string() {
    let id = DocumentPartId(DOC_PART_ID_VALUE.to_string());

    assert_eq!(DOC_PART_ID_VALUE, &*id);
    assert_eq!(DOC_PART_ID_VALUE, id.as_string());
}

#[test]
fn test_index_id_deref_and_as_string() {
    let id = IndexId(INDEX_ID_VALUE.to_string());

    assert_eq!(INDEX_ID_VALUE, &*id);
    assert_eq!(INDEX_ID_VALUE, id.as_string());
}
