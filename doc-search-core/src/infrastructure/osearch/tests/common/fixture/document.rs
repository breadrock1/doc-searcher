use crate::infrastructure::osearch::dto::{FoundedDocumentInfo, IndexInformation, SourceDocument};
use crate::infrastructure::osearch::tests::fixture::INDEX_ID;
use crate::infrastructure::osearch::tests::fixture::search::build_full_search_result;
use crate::shared::kernel::metadata::{
    DocumentClassBuilder, DocumentGroup, DocumentIcon, DocumentLocationBuilder, DocumentMetadata,
    DocumentMetadataBuilder, DocumentReference, DocumentSubject, PipelineLabel,
};
use rstest::fixture;

pub fn build_document_metadata() -> anyhow::Result<DocumentMetadata> {
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

pub fn first_hit_source() -> serde_json::Value {
    build_full_search_result()["hits"]["hits"][0]["_source"].clone()
}

#[fixture]
pub fn build_source_document() -> SourceDocument {
    serde_json::from_value(first_hit_source()).expect("failed to parse first hit")
}

#[fixture]
pub fn build_founded_document_info() -> FoundedDocumentInfo {
    let hit = build_full_search_result()["hits"]["hits"][0].clone();
    serde_json::from_value(hit).expect("failed to parse founded document info")
}

#[fixture]
pub fn build_index_info() -> IndexInformation {
    serde_json::from_value(serde_json::json!({ "index": INDEX_ID })).expect("failed to parse index")
}
