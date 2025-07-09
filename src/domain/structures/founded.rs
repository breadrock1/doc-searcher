use crate::domain::Document;

pub struct FoundedDocument {
    pub id: String,
    pub document: Document,
    pub score: Option<f64>,
    pub highlight: Vec<String>,
}
