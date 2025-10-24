use anyhow;
use rstest::rstest;
use serde_json::Value;

use crate::infrastructure::osearch::extractor::extract_founded_docs;
use crate::infrastructure::osearch::tests::fixture::search::build_osearch_search_result;
use crate::infrastructure::osearch::tests::fixture::search::{DOCUMENT_ID, INDEX_ID, SCROLL_ID};

#[rstest]
fn test_extract_founded_docs(
    #[from(build_osearch_search_result)] founded: Value,
) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(&founded).unwrap());
    let extracted_docs = extract_founded_docs(founded)?;

    let scroll_id = extracted_docs
        .scroll_id()
        .as_ref()
        .expect("expected scroll_id");
    assert_eq!(SCROLL_ID, scroll_id);
    assert_eq!(1, extracted_docs.founded().len());

    let root_document = extracted_docs.founded().first().expect("expected document");
    assert_eq!(DOCUMENT_ID, root_document.id());
    assert_eq!(INDEX_ID, root_document.folder_id());

    Ok(())
}

#[rstest]
fn test_extract_founded_docs_without_scroll_id(
    #[from(build_osearch_search_result)] mut founded: Value,
) -> anyhow::Result<()> {
    founded[&"_scroll_id"] = Value::Null;
    println!("{}", serde_json::to_string_pretty(&founded).unwrap());
    let extracted_docs = extract_founded_docs(founded)?;

    assert!(extracted_docs.scroll_id().is_none());
    assert_eq!(1, extracted_docs.founded().len());

    let root_document = extracted_docs.founded().first().expect("expected document");
    assert_eq!(DOCUMENT_ID, root_document.id());
    assert_eq!(INDEX_ID, root_document.folder_id());

    Ok(())
}
