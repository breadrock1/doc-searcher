#![allow(dead_code, dead_code)]
use derive_builder::Builder;

#[derive(Debug, Builder)]
pub struct DocumentMetadata {
    pub photo: Option<String>,
    pub source: Option<String>,
    pub semantic_source: Option<String>,
    pub summary: Option<String>,
    pub locations: Vec<DocumentLocation>,
    pub subjects: Vec<DocumentSubject>,
    pub classes: Vec<DocumentClass>,
    pub icons: Vec<DocumentIcons>,
    pub groups: Vec<DocumentGroups>,
    pub pipelines: Vec<PipelineLabel>,
    pub references: Vec<DocumentReference>,
}

#[derive(Clone, Debug)]
pub struct DocumentReference(String);

#[derive(Clone, Debug)]
pub struct PipelineLabel(String);

#[derive(Clone, Debug, Builder)]
pub struct DocumentLocation {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Debug, Builder)]
pub struct DocumentSubject {
    pub name: String,
}

#[derive(Clone, Debug, Builder)]
pub struct DocumentClass {
    pub name: String,
    pub probability: f64,
}

#[derive(Clone, Debug, Builder)]
pub struct DocumentIcons {
    pub name: String,
}

#[derive(Clone, Debug, Builder)]
pub struct DocumentGroups {
    pub name: String,
}
