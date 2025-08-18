use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Clone, Getters, CopyGetters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct Document {
    file_name: String,
    file_path: String,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    file_size: u32,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    created_at: i64,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    modified_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunked_text: Option<Vec<String>>,
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
