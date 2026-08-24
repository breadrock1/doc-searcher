use anyhow::Result;

use crate::domain::searcher::models::DocumentPartEntrails;
use crate::domain::storage::models::{DocumentPart, DocumentPartBuilder};
use crate::infrastructure::osearch::dto::{
    Class, FoundedDocumentInfo, Group, Icons, IndexInformation, Location, Pipeline, Reference,
    SourceDocument, SourceDocumentMetadata, Subject,
};
use crate::infrastructure::osearch::tests::fixture::search::build_full_search_result;
use crate::infrastructure::osearch::tests::fixture::{DOCUMENT_ID, INDEX_ID};
use crate::shared::kernel::metadata::{
    DocumentClassBuilder, DocumentGroup, DocumentIcon, DocumentLocationBuilder, DocumentMetadata,
    DocumentMetadataBuilder, DocumentReference, DocumentSubject, PipelineLabel,
};
use crate::shared::kernel::{IndexId, LargeDocumentId};

fn build_document_metadata() -> Result<DocumentMetadata> {
    Ok(DocumentMetadataBuilder::default()
        .pipeline_id(Some(123))
        .photo(Some("photo.jpg".to_string()))
        .source(Some("source".to_string()))
        .semantic_source(Some("semantic".to_string()))
        .summary(Some("summary".to_string()))
        .locations(vec![
            DocumentLocationBuilder::default()
                .name("Moscow".to_string())
                .latitude(55.75)
                .longitude(37.61)
                .build()?,
        ])
        .subjects(vec![DocumentSubject("politics".to_string())])
        .classes(vec![
            DocumentClassBuilder::default()
                .name("news".to_string())
                .probability(0.9)
                .build()?,
        ])
        .icons(vec![DocumentIcon("icon".to_string())])
        .groups(vec![DocumentGroup("group".to_string())])
        .pipelines(vec![PipelineLabel("ml-pipeline".to_string())])
        .references(vec![DocumentReference("ref-1".to_string())])
        .build()?)
}

fn first_hit_source() -> serde_json::Value {
    build_full_search_result()["hits"]["hits"][0]["_source"].clone()
}

#[test]
fn test_source_document_to_document_part_entrails() -> Result<()> {
    let source: SourceDocument = serde_json::from_value(first_hit_source())?;

    let entrails: DocumentPartEntrails = source.try_into()?;
    assert_eq!(DOCUMENT_ID, entrails.large_doc_id.0.as_str());
    assert!(entrails.content.is_some());

    let meta = entrails.metadata.expect("metadata must be converted");
    assert_eq!("source", meta.source.as_deref().expect("source"));
    Ok(())
}

#[test]
fn test_document_part_to_source_document() -> Result<()> {
    let doc_part = DocumentPartBuilder::default()
        .large_doc_id(LargeDocumentId(DOCUMENT_ID.to_string()))
        .doc_part_id(1)
        .file_name("name.txt".to_string())
        .file_path("/path/name.txt".to_string())
        .file_size(100)
        .created_at(1)
        .modified_at(2)
        .content("hello world".to_string())
        .metadata(Some(build_document_metadata()?))
        .build()?;

    let source: SourceDocument = doc_part.try_into()?;
    assert_eq!(DOCUMENT_ID, source.large_doc_id);
    assert_eq!(Some("hello world".to_string()), source.content);
    assert_eq!(100, source.file_size);
    assert!(source.metadata.is_some());
    Ok(())
}

#[test]
fn test_founded_document_info_to_document_part() -> Result<()> {
    let hit = build_full_search_result()["hits"]["hits"][0].clone();
    let info: FoundedDocumentInfo = serde_json::from_value(hit)?;

    let doc_part: DocumentPart = info.try_into()?;
    assert_eq!(DOCUMENT_ID, doc_part.large_doc_id.0.as_str());
    assert_eq!("There is some highlight content", doc_part.content);
    Ok(())
}

#[test]
fn test_index_information_to_index_id() -> Result<()> {
    let info: IndexInformation = serde_json::from_value(serde_json::json!({ "index": INDEX_ID }))?;

    let index_id: IndexId = info.into();
    assert_eq!(INDEX_ID, index_id.0.as_str());
    Ok(())
}

#[test]
fn test_document_metadata_to_source_document_metadata() -> Result<()> {
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
fn test_source_document_metadata_to_document_metadata() -> Result<()> {
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
