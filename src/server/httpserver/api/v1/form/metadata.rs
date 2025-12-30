use anyhow::Context;
use derive_builder::Builder;
use doc_search_core::shared::kernel::metadata::{DocumentClass, DocumentClassBuilder};
use doc_search_core::shared::kernel::metadata::{
    DocumentGroup, DocumentIcon, DocumentReference, DocumentSubject, PipelineLabel,
};
use doc_search_core::shared::kernel::metadata::{DocumentLocation, DocumentLocationBuilder};
use doc_search_core::shared::kernel::metadata::{DocumentMetadata, DocumentMetadataBuilder};
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Metadata {
    pub photo: Option<String>,
    pub source: Option<String>,
    pub semantic_source: Option<String>,
    pub summary: Option<String>,
    pub locations: Option<Vec<Location>>,
    pub subjects: Option<Vec<Subject>>,
    pub classes: Option<Vec<Class>>,
    pub icons: Option<Vec<Icons>>,
    pub groups: Option<Vec<Group>>,
    pub pipelines: Option<Vec<String>>,
    pub references: Option<Vec<String>>,
}

impl TryFrom<Metadata> for DocumentMetadata {
    type Error = anyhow::Error;

    fn try_from(metadata: Metadata) -> Result<Self, Self::Error> {
        let locations = metadata
            .locations
            .unwrap_or_default()
            .into_iter()
            .map(Location::into)
            .collect();

        let subjects = metadata
            .subjects
            .unwrap_or_default()
            .into_iter()
            .map(Subject::into)
            .collect();

        let classes = metadata
            .classes
            .unwrap_or_default()
            .into_iter()
            .map(Class::into)
            .collect();

        let icons = metadata
            .icons
            .unwrap_or_default()
            .into_iter()
            .map(Icons::into)
            .collect();

        let groups = metadata
            .groups
            .unwrap_or_default()
            .into_iter()
            .map(Group::into)
            .collect();

        let pipelines = metadata
            .pipelines
            .unwrap_or_default()
            .into_iter()
            .map(PipelineLabel)
            .collect();

        let references = metadata
            .references
            .unwrap_or_default()
            .into_iter()
            .map(DocumentReference)
            .collect();

        DocumentMetadataBuilder::default()
            .photo(metadata.photo)
            .source(metadata.source)
            .semantic_source(metadata.semantic_source)
            .summary(metadata.summary)
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

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Location {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<Location> for DocumentLocation {
    fn from(location: Location) -> Self {
        DocumentLocationBuilder::default()
            .name(location.name)
            .latitude(location.latitude)
            .longitude(location.longitude)
            .build()
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Subject {
    pub name: String,
}

impl From<Subject> for DocumentSubject {
    fn from(subject: Subject) -> Self {
        DocumentSubject(subject.name)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
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
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Icons {
    pub name: String,
}

impl From<Icons> for DocumentIcon {
    fn from(icon: Icons) -> Self {
        DocumentIcon(icon.name)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct Group {
    pub name: String,
}

impl From<Group> for DocumentGroup {
    fn from(group: Group) -> Self {
        DocumentGroup(group.name)
    }
}
