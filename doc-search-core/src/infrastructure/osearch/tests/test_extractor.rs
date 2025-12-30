use anyhow;
use rstest::rstest;
use serde_json::Value;

use crate::infrastructure::osearch::extractor::extract_founded_document_parts;
use crate::infrastructure::osearch::tests::fixture::search::*;
use crate::infrastructure::osearch::tests::fixture::{
    DOCUMENT_ID, DOCUMENT_PART_ID, INDEX_ID, SCROLL_ID,
};

#[rstest]
#[case(build_full_search_result(), Some(SCROLL_ID.to_string()))]
#[case(build_search_result_without_metadata(), Some(SCROLL_ID.to_string()))]
#[case(build_search_result_without_scroll_id(), None)]
fn test_extract_founded_docs(
    #[case] founded: Value,
    #[case] expected_scroll_id: Option<String>,
) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(&founded)?);
    let extracted_docs = extract_founded_document_parts(founded)?;

    let scroll_id_opt = extracted_docs.scroll_id;
    assert_eq!(expected_scroll_id, scroll_id_opt);
    assert_eq!(1, extracted_docs.founded.len());

    let root_document = extracted_docs.founded.first().expect("expected document");
    assert_eq!(DOCUMENT_ID, root_document.document.large_doc_id);
    assert_eq!(DOCUMENT_PART_ID, root_document.id);
    assert_eq!(INDEX_ID, root_document.index);

    Ok(())
}
