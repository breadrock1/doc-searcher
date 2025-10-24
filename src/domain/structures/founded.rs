use crate::domain::DocumentPart;

pub struct FoundedDocument {
    pub id: String,
    pub folder_id: String,
    pub document: DocumentPart,
    pub score: Option<f64>,
    pub highlight: Vec<String>,
}
