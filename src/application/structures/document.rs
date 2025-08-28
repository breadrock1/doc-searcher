use derive_builder::Builder;
use gset::Getset;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Clone, Getset, Serialize, Deserialize)]
pub struct Document {
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
    #[getset(get, vis = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[getset(get, vis = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    chunked_text: Option<Vec<String>>,
    #[getset(get, vis = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    embeddings: Option<Vec<Embeddings>>,
}

impl TryFrom<crate::domain::Document> for Document {
    type Error = anyhow::Error;

    fn try_from(value: crate::domain::Document) -> Result<Self, Self::Error> {
        let document = DocumentBuilder::default()
            .file_name(value.file_name)
            .file_path(value.file_path)
            .file_size(value.file_size)
            .content(Some(value.content))
            .created_at(value.created_at)
            .modified_at(value.modified_at)
            .build()?;

        Ok(document)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Embeddings {
    pub knn: Vec<f64>,
}

impl Embeddings {
    pub fn new(knn: Vec<f64>) -> Self {
        Self { knn }
    }
}

pub struct StoredDocument {
    pub id: String,
    pub file_path: String,
}

impl StoredDocument {
    pub fn new(id: String, file_path: String) -> Self {
        StoredDocument { id, file_path }
    }
}
