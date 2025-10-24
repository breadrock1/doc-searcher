use derive_builder::Builder;
use gset::Getset;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Getset, Serialize, Deserialize)]
pub struct DocumentPart {
    #[getset(get, vis = "pub")]
    file_name: String,
    #[getset(get, vis = "pub")]
    file_path: String,
    #[getset(get_copy, vis = "pub")]
    file_size: u32,
    #[getset(get_copy, vis = "pub")]
    created_at: i64,
    #[getset(get_copy, vis = "pub")]
    modified_at: i64,
    #[getset(set, vis = "pub")]
    #[getset(get, vis = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[getset(get, vis = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    chunked_text: Option<Vec<String>>,
    #[getset(get, vis = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    embeddings: Option<Vec<Embeddings>>,
    #[getset(set, vis = "pub")]
    #[getset(get_copy, vis = "pub")]
    doc_part_id: usize,
}

impl TryFrom<crate::domain::DocumentPart> for DocumentPart {
    type Error = anyhow::Error;

    fn try_from(value: crate::domain::DocumentPart) -> Result<Self, Self::Error> {
        let document = DocumentPartBuilder::default()
            .file_name(value.file_name)
            .file_path(value.file_path)
            .file_size(value.file_size)
            .content(Some(value.content))
            .created_at(value.created_at)
            .modified_at(value.modified_at)
            .doc_part_id(value.doc_part_id)
            .build()?;

        Ok(document)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embeddings {
    pub knn: Vec<f64>,
}

impl Embeddings {
    pub fn new(knn: Vec<f64>) -> Self {
        Self { knn }
    }
}

#[derive(Clone)]
pub struct StoredDocumentPart {
    pub id: String,
    pub file_path: String,
}

impl StoredDocumentPart {
    pub fn new(id: String, file_path: String) -> Self {
        StoredDocumentPart { id, file_path }
    }
}
