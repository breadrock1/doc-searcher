use derive_builder::Builder;

#[derive(Clone, Debug, Builder)]
pub struct DocumentMetadata {
    pub photo: Option<String>,
    pub source: Option<String>,
    pub semantic_source: Option<String>,
    pub summary: Option<String>,
    pub locations: Vec<DocumentLocation>,
    pub subjects: Vec<DocumentSubject>,
    pub classes: Vec<DocumentClass>,
    pub icons: Vec<DocumentIcon>,
    pub groups: Vec<DocumentGroup>,
    pub pipelines: Vec<PipelineLabel>,
    pub references: Vec<DocumentReference>,
}

#[derive(Clone, Debug)]
pub struct DocumentIcon(pub String);

#[derive(Clone, Debug)]
pub struct DocumentSubject(pub String);

#[derive(Clone, Debug)]
pub struct DocumentReference(pub String);

#[derive(Clone, Debug)]
pub struct PipelineLabel(pub String);

#[derive(Clone, Debug)]
pub struct DocumentGroup(pub String);

#[derive(Clone, Debug, Builder)]
pub struct DocumentLocation {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Debug, Builder)]
pub struct DocumentClass {
    pub name: String,
    pub probability: f64,
}
