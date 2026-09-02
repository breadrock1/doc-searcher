use rstest::rstest;

use crate::domain::searcher::models::DocumentPartEntrails;
use crate::domain::storage::models::DocumentPart;
use crate::domain::storage::tests::fixture::document::*;
use crate::infrastructure::osearch::dto::*;
use crate::infrastructure::osearch::tests::fixture::document::*;
use crate::infrastructure::osearch::tests::fixture::{DOCUMENT_ID, INDEX_ID};
use crate::shared::kernel::IndexId;
use crate::shared::kernel::metadata::DocumentMetadata;

#[rstest]
fn test_source_document_to_document_part_entrails(
    #[from(build_source_document)] source_doc: SourceDocument,
) -> anyhow::Result<()> {
    let entrails: DocumentPartEntrails = source_doc.try_into()?;
    assert_eq!(DOCUMENT_ID, entrails.large_doc_id.0.as_str());

    let meta = entrails.metadata.expect("metadata must be converted");
    assert_eq!("source", meta.source.as_deref().expect("source"));
    Ok(())
}

#[rstest]
fn test_document_part_to_source_document() -> anyhow::Result<()> {
    let doc_part = build_document_part(1);
    let source: SourceDocument = doc_part.try_into()?;
    assert_eq!(LARGE_DOCUMENT_ID, source.large_doc_id);
    assert_eq!(Some(LARGE_DOCUMENT_CONTENT.to_string()), source.content);
    assert_eq!(LARGE_DOCUMENT_FILE_SIZE, source.file_size);
    Ok(())
}

#[rstest]
fn test_founded_document_info_to_document_part(
    #[from(build_founded_document_info)] doc_info: FoundedDocumentInfo,
) -> anyhow::Result<()> {
    let doc_part: DocumentPart = doc_info.try_into()?;
    assert_eq!(DOCUMENT_ID, doc_part.large_doc_id.0.as_str());
    assert_eq!("There is some highlight content", doc_part.content);
    Ok(())
}

#[rstest]
fn test_index_information_to_index_id(
    #[from(build_index_info)] index_info: IndexInformation,
) -> anyhow::Result<()> {
    let index_id: IndexId = index_info.into();
    assert_eq!(INDEX_ID, index_id.0.as_str());
    Ok(())
}

#[test]
fn test_document_metadata_to_source_document_metadata() -> anyhow::Result<()> {
    let meta = build_document_metadata()?;

    let src: SourceDocumentMetadata = meta.try_into()?;
    assert_eq!("Moscow", src.locations[0].name);
    assert_eq!(vec![55.75, 37.61], src.locations[0].coords);
    assert_eq!("politics", src.subjects[0].name);
    assert_eq!("news", src.classes[0].name);
    assert_eq!(0.9, src.classes[0].probability);
    assert_eq!("icon", src.icons[0].name);
    assert_eq!("group", src.groups[0].name);
    assert_eq!("ml-pipeline", src.pipelines[0].0);
    assert_eq!("ref-1", src.references[0].0);
    Ok(())
}

#[test]
fn test_source_document_metadata_to_document_metadata() -> anyhow::Result<()> {
    let src = SourceDocumentMetadata {
        photo: Some("photo.jpg".to_string()),
        pipeline_id: Some(123),
        source: Some("source".to_string()),
        semantic_source: Some("semantic".to_string()),
        summary: Some("summary".to_string()),
        locations: vec![Location {
            name: "Moscow".to_string(),
            coords: vec![55.75, 37.61],
        }],
        subjects: vec![Subject {
            name: "politics".to_string(),
        }],
        classes: vec![Class {
            name: "news".to_string(),
            probability: 0.9,
        }],
        icons: vec![Icons {
            name: "icon".to_string(),
        }],
        groups: vec![Group {
            name: "group".to_string(),
        }],
        pipelines: vec![Pipeline("ml-pipeline".to_string())],
        references: vec![Reference("ref-1".to_string())],
    };

    let meta: DocumentMetadata = src.try_into()?;
    assert_eq!("Moscow", meta.locations[0].name);
    assert_eq!(37.61, meta.locations[0].latitude);
    assert_eq!(55.75, meta.locations[0].longitude);
    assert_eq!("politics", meta.subjects[0].0);
    assert_eq!("news", meta.classes[0].name);
    assert_eq!("icon", meta.icons[0].0);
    assert_eq!("group", meta.groups[0].0);
    assert_eq!("ml-pipeline", meta.pipelines[0].0);
    assert_eq!("ref-1", meta.references[0].0);
    Ok(())
}
