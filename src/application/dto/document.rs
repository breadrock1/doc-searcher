use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(unused_imports)]
use serde_json::json;

#[derive(Builder, Clone, Getters, CopyGetters, Serialize, Deserialize, ToSchema)]
pub struct Document {
    #[schema(example = "test-document.docx")]
    #[getset(get = "pub")]
    file_name: String,
    #[schema(example = "./test-document.docx")]
    #[getset(get = "pub")]
    file_path: String,
    #[schema(example = 1024)]
    #[getset(get_copy = "pub")]
    file_size: u32,
    #[schema(example = "There is some content data")]
    #[getset(get = "pub")]
    content: Option<String>,
    #[schema(example = 1750957115)]
    #[getset(get = "pub")]
    created_at: i64,
    #[schema(example = 1750957115)]
    #[getset(get = "pub")]
    modified_at: i64,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::default()
    }
}

impl TryFrom<crate::domain::Document> for Document {
    type Error = DocumentBuilderError;

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
