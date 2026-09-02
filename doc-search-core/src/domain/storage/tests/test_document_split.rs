use rstest::rstest;

use crate::domain::storage::models::LargeDocument;
use crate::domain::storage::tests::fixture::document::{build_document_part, build_large_document};

#[rstest]
#[case(build_large_document(), 0, 1)]
#[case(build_large_document(), 2, 1)]
#[case(build_large_document(), 10, 1)]
fn test_build_searching_params(
    #[case] large_document: LargeDocument,
    #[case] max_content_size: usize,
    #[case] expected_doc_parts: usize,
) -> anyhow::Result<()> {
    let document_parts = large_document.divide_large_document_on_parts(max_content_size)?;
    assert_eq!(document_parts.len(), expected_doc_parts);
    Ok(())
}

#[test]
fn test_large_document_and_document_part_debug() {
    let large_document = build_large_document();
    let large_debug = format!("{large_document:?}");
    assert!(large_debug.contains("file_path"));
    assert!(large_debug.contains("create_at"));

    let doc_part = build_document_part(1);
    let part_debug = format!("{doc_part:?}");
    assert!(part_debug.contains("large_doc_id"));
    assert!(part_debug.contains("doc_part_id"));
}
