use rstest::fixture;

use crate::application::tests::fixture::*;
use crate::domain::storage::models::LargeDocument;
use crate::domain::storage::models::LargeDocumentBuilder;
use crate::shared::kernel::metadata::*;

#[fixture]
pub fn build_large_document() -> LargeDocument {
    LargeDocumentBuilder::default()
        .file_name(DOC_FILE_NAME.to_string())
        .file_path(DOC_FILE_PATH.to_string())
        .file_size(DOC_FILE_SIZE)
        .created_at(DOC_FILE_TIMESTAMP)
        .modified_at(DOC_FILE_TIMESTAMP)
        .content(DOC_FILE_LARGE_CONTENT.to_string())
        .metadata(Some(build_document_metadata()))
        .build()
        .expect("failed to build large document fixture")
}

#[fixture]
pub fn build_short_document() -> LargeDocument {
    LargeDocumentBuilder::default()
        .file_name(DOC_FILE_NAME.to_string())
        .file_path(DOC_FILE_PATH.to_string())
        .file_size(DOC_FILE_SIZE)
        .created_at(DOC_FILE_TIMESTAMP)
        .modified_at(DOC_FILE_TIMESTAMP)
        .content(DOC_FILE_SHORT_CONTENT.to_string())
        .metadata(None)
        .build()
        .expect("failed to build short document fixture")
}

#[fixture]
fn build_document_metadata() -> DocumentMetadata {
    DocumentMetadataBuilder::default()
        .photo(Some(PHOTO_PATH.to_string()))
        .pipeline_id(Some(DOCUMENT_PIPELINE_ID))
        .source(Some(DOCUMENT_SOURCE.to_string()))
        .semantic_source(Some(DOCUMENT_SEMANTIC_SOURCE.to_string()))
        .summary(Some(DOCUMENT_SUMMARY_SOURCE.to_string()))
        .locations(build_metadata_locations())
        .subjects(build_metadata_subjects())
        .classes(build_metadata_classes())
        .icons(build_metadata_icons())
        .groups(build_metadata_groups())
        .pipelines(build_metadata_pipelines())
        .references(build_metadata_references())
        .build()
        .expect("failed to build metadata")
}

#[fixture]
fn build_metadata_subjects() -> Vec<DocumentSubject> {
    vec![DocumentSubject(DOCUMENT_SUBJECT_NAME.to_string())]
}

#[fixture]
fn build_metadata_groups() -> Vec<DocumentGroup> {
    vec![DocumentGroup(DOCUMENT_GROUP_NAME.to_string())]
}

#[fixture]
fn build_metadata_icons() -> Vec<DocumentIcon> {
    vec![DocumentIcon(DOCUMENT_ICON_NAME.to_string())]
}

#[fixture]
fn build_metadata_pipelines() -> Vec<PipelineLabel> {
    vec![PipelineLabel(DOCUMENT_PIPELINE_NAME.to_string())]
}

#[fixture]
fn build_metadata_references() -> Vec<DocumentReference> {
    vec![DocumentReference(DOCUMENT_REFERENCE_NAME.to_string())]
}

#[fixture]
fn build_metadata_locations() -> Vec<DocumentLocation> {
    let location = DocumentLocationBuilder::default()
        .name(DOCUMENT_LOCATION_NAME.to_string())
        .latitude(0f64)
        .longitude(0f64)
        .build()
        .expect("built document location failed");

    vec![location]
}

#[fixture]
fn build_metadata_classes() -> Vec<DocumentClass> {
    let class = DocumentClassBuilder::default()
        .name(DOCUMENT_CLASS_NAME.to_string())
        .probability(8f64)
        .build()
        .expect("built document class failed");

    vec![class]
}
