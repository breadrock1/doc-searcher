use derive_builder::Builder;

use crate::shared::kernel::metadata::DocumentMetadata;

#[derive(Clone, Builder)]
pub struct FoundedDocument {
    pub id: String,
    pub index: String,
    pub score: Option<f64>,
    pub highlight: Vec<String>,
    pub document: DocumentPartEntrails,
}

#[derive(Clone, Default, Debug, Builder)]
pub struct DocumentPartEntrails {
    pub large_doc_id: String,
    pub doc_part_id: usize,
    pub file_name: String,
    pub file_path: String,
    pub file_size: u32,
    pub created_at: i64,
    pub modified_at: i64,
    pub content: Option<String>,
    pub chunked_text: Option<Vec<String>>,
    pub embeddings: Option<Vec<Embeddings>>,
    pub metadata: Option<DocumentMetadata>,
}

#[derive(Clone, Debug)]
pub struct Embeddings {
    pub knn: Vec<f64>,
}

impl From<Vec<f64>> for Embeddings {
    fn from(tokens: Vec<f64>) -> Self {
        Embeddings { knn: tokens }
    }
}
