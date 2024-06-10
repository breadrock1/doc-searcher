use crate::forms::documents::document::Document;
use crate::forms::documents::metadata::Artifacts;
use crate::forms::documents::DocumentsTrait;
use crate::forms::TestExample;

use datetime::deserialize_dt;
use datetime::serialize_dt;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct DocumentPreview {
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    id: String,
    #[schema(example = "test_document.txt")]
    name: String,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-03T13:51:32Z")]
    created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quality_recognition: Option<i32>,
    #[schema(example = 35345)]
    file_size: i32,
    #[schema(example = "Test Folder")]
    location: String,
    #[schema(example = "test_folder")]
    folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview_properties: Option<Vec<Artifacts>>,
}

#[allow(dead_code)]
impl DocumentPreview {
    pub fn builder() -> DocumentPreviewBuilder {
        DocumentPreviewBuilder::default()
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn set_folder_id(&mut self, folder_id: &str) {
        self.folder_id = folder_id.to_string();
    }
    pub fn get_location(&self) -> &str {
        self.location.as_str()
    }
    pub fn get_artifacts(&self) -> Option<&Vec<Artifacts>> {
        self.preview_properties.as_ref()
    }
    pub fn get_quality(&self) -> Option<i32> {
        self.quality_recognition
    }
    pub fn get_size(&self) -> i32 {
        self.file_size
    }
    pub fn get_created_date(&self) -> Option<&DateTime<Utc>> {
        self.created_at.as_ref()
    }
}

impl DocumentsTrait for DocumentPreview {
    fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }
    fn get_doc_id(&self) -> &str {
        self.id.as_str()
    }
    fn set_folder_id(&mut self, folder_id: &str) {
        self.folder_id = folder_id.to_string()
    }
}

impl TestExample<DocumentPreview> for DocumentPreview {
    fn test_example(_val: Option<&str>) -> DocumentPreview {
        let document = Document::test_example(None);
        DocumentPreview::from(&document)
    }
}

impl From<&Document> for DocumentPreview {
    fn from(value: &Document) -> Self {
        let artifacts = match value.get_ocr_metadata() {
            Some(metadata) => metadata.get_artifacts().cloned(),
            None => None,
        };

        DocumentPreview::builder()
            .id(value.get_doc_id().to_string())
            .folder_id(value.get_folder_id().to_string())
            .name(value.get_doc_name().to_string())
            .location(value.get_folder_id().to_string())
            .created_at(value.get_doc_created().cloned())
            .quality_recognition(value.get_ocr_quality())
            .file_size(value.get_doc_size())
            .preview_properties(artifacts)
            .build()
            .unwrap()
    }
}
