use anyhow::Context;
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

use crate::shared::kernel::metadata::{
    DocumentClass, DocumentClassBuilder, DocumentGroup, DocumentIcon, DocumentLocation,
    DocumentLocationBuilder, DocumentMetadata, DocumentMetadataBuilder, DocumentReference,
    DocumentSubject, PipelineLabel,
};

#[derive(Clone, Deserialize, Serialize)]
pub struct SourceDocumentMetadata {
    pub photo: Option<String>,
    pub pipeline_id: Option<i64>,
    pub source: Option<String>,
    pub semantic_source: Option<String>,
    pub summary: Option<String>,
    pub locations: Vec<Location>,
    pub subjects: Vec<Subject>,
    pub classes: Vec<Class>,
    pub icons: Vec<Icons>,
    pub groups: Vec<Group>,
    pub pipelines: Vec<Pipeline>,
    pub references: Vec<Reference>,
}

impl TryFrom<DocumentMetadata> for SourceDocumentMetadata {
    type Error = anyhow::Error;

    fn try_from(doc_metadata: DocumentMetadata) -> Result<Self, Self::Error> {
        let locations = doc_metadata
            .locations
            .into_iter()
            .map(Location::from)
            .collect();

        let subjects = doc_metadata
            .subjects
            .into_iter()
            .map(Subject::from)
            .collect();

        let classes = doc_metadata.classes.into_iter().map(Class::from).collect();

        let icons = doc_metadata.icons.into_iter().map(Icons::from).collect();

        let groups = doc_metadata.groups.into_iter().map(Group::from).collect();

        let pipelines = doc_metadata
            .pipelines
            .into_iter()
            .map(Pipeline::from)
            .collect();

        let references = doc_metadata
            .references
            .into_iter()
            .map(Reference::from)
            .collect();

        Ok(SourceDocumentMetadata {
            photo: doc_metadata.photo,
            pipeline_id: doc_metadata.pipeline_id,
            source: doc_metadata.source,
            semantic_source: doc_metadata.semantic_source,
            summary: doc_metadata.summary,
            locations,
            subjects,
            classes,
            icons,
            groups,
            pipelines,
            references,
        })
    }
}

impl TryFrom<SourceDocumentMetadata> for DocumentMetadata {
    type Error = anyhow::Error;

    fn try_from(src_metadata: SourceDocumentMetadata) -> Result<Self, Self::Error> {
        let locations = src_metadata
            .locations
            .into_iter()
            .map(Location::into)
            .collect();

        let subjects = src_metadata
            .subjects
            .into_iter()
            .map(Subject::into)
            .collect();

        let classes = src_metadata.classes.into_iter().map(Class::into).collect();

        let icons = src_metadata.icons.into_iter().map(Icons::into).collect();

        let groups = src_metadata.groups.into_iter().map(Group::into).collect();

        let pipelines = src_metadata
            .pipelines
            .into_iter()
            .map(Pipeline::into)
            .collect();

        let references = src_metadata
            .references
            .into_iter()
            .map(Reference::into)
            .collect();

        DocumentMetadataBuilder::default()
            .photo(src_metadata.photo)
            .pipeline_id(src_metadata.pipeline_id)
            .source(src_metadata.source)
            .semantic_source(src_metadata.semantic_source)
            .summary(src_metadata.summary)
            .locations(locations)
            .subjects(subjects)
            .classes(classes)
            .icons(icons)
            .groups(groups)
            .pipelines(pipelines)
            .references(references)
            .build()
            .context("failed to build metadata")
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Reference(pub String);

impl From<Reference> for DocumentReference {
    fn from(reference: Reference) -> Self {
        DocumentReference(reference.0)
    }
}

impl From<DocumentReference> for Reference {
    fn from(reference: DocumentReference) -> Self {
        Reference(reference.0)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Pipeline(pub String);

impl From<Pipeline> for PipelineLabel {
    fn from(pipeline: Pipeline) -> Self {
        PipelineLabel(pipeline.0)
    }
}

impl From<PipelineLabel> for Pipeline {
    fn from(pipeline: PipelineLabel) -> Self {
        Pipeline(pipeline.0)
    }
}

#[derive(Clone, Builder, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub coords: Vec<f64>,
}

impl From<Location> for DocumentLocation {
    fn from(location: Location) -> Self {
        DocumentLocationBuilder::default()
            .name(location.name)
            .longitude(location.coords[0])
            .latitude(location.coords[1])
            .build()
            .expect("converting location to document location failed")
    }
}

impl From<DocumentLocation> for Location {
    fn from(location: DocumentLocation) -> Self {
        Location {
            name: location.name,
            coords: vec![location.latitude, location.longitude],
        }
    }
}

#[derive(Clone, Builder, Deserialize, Serialize)]
pub struct Subject {
    pub name: String,
}

impl From<Subject> for DocumentSubject {
    fn from(subject: Subject) -> Self {
        DocumentSubject(subject.name)
    }
}

impl From<DocumentSubject> for Subject {
    fn from(subject: DocumentSubject) -> Self {
        Subject { name: subject.0 }
    }
}

#[derive(Clone, Builder, Deserialize, Serialize)]
pub struct Class {
    pub name: String,
    pub probability: f64,
}

impl From<Class> for DocumentClass {
    fn from(class: Class) -> Self {
        DocumentClassBuilder::default()
            .name(class.name)
            .probability(class.probability)
            .build()
            .expect("converting class to document class failed")
    }
}

impl From<DocumentClass> for Class {
    fn from(class: DocumentClass) -> Self {
        Class {
            name: class.name,
            probability: class.probability,
        }
    }
}

#[derive(Clone, Builder, Deserialize, Serialize)]
pub struct Icons {
    pub name: String,
}

impl From<Icons> for DocumentIcon {
    fn from(icon: Icons) -> Self {
        DocumentIcon(icon.name)
    }
}

impl From<DocumentIcon> for Icons {
    fn from(icon: DocumentIcon) -> Self {
        Icons { name: icon.0 }
    }
}

#[derive(Clone, Builder, Deserialize, Serialize)]
pub struct Group {
    pub name: String,
}

impl From<Group> for DocumentGroup {
    fn from(group: Group) -> Self {
        DocumentGroup(group.name)
    }
}

impl From<DocumentGroup> for Group {
    fn from(group: DocumentGroup) -> Self {
        Group { name: group.0 }
    }
}
