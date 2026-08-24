use crate::domain::searcher::models::{
    DocumentPartEntrailsBuilder, Embeddings, FoundedDocumentBuilder,
};
use crate::shared::kernel::LargeDocumentId;

const LARGE_DOC_ID: &str = "large-doc-1";

#[test]
fn test_embeddings_from_vec() {
    let tokens = vec![0.1_f64, 0.2, 0.3];
    let embeddings: Embeddings = tokens.clone().into();
    assert_eq!(tokens, embeddings.knn);
}

#[test]
fn test_founded_document_debug() {
    let document_part = DocumentPartEntrailsBuilder::default()
        .large_doc_id(LargeDocumentId(LARGE_DOC_ID.to_string()))
        .doc_part_id(1)
        .file_name("file.pdf".to_string())
        .file_path("/some/path/file.pdf".to_string())
        .file_size(1024)
        .created_at(1_756_498_133)
        .modified_at(1_756_498_133)
        .content(Some("document content".to_string()))
        .chunked_text(None)
        .embeddings(None)
        .metadata(None)
        .build()
        .expect("failed to build document part entrails");

    let found = FoundedDocumentBuilder::default()
        .id("doc-1".to_string())
        .index("index-1".to_string())
        .score(Some(0.9))
        .highlight(vec!["term".to_string()])
        .document(document_part)
        .build()
        .expect("failed to build founded document");

    let debug = format!("{found:?}");
    assert!(debug.contains("id: doc-1"));
    assert!(debug.contains("index: index-1"));
    assert!(debug.contains("large_doc_id"));
    assert!(debug.contains("doc_part_id: 1"));
}
