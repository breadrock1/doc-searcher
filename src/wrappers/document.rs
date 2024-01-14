use chrono::{DateTime, Utc};
use datetime::{deserialize_dt, serialize_dt};
use derive_builder::Builder;
use loader::FileData;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Builder, Default)]
pub struct Document {
    pub bucket_uuid: String,
    pub bucket_path: String,
    pub document_name: String,
    pub document_path: String,
    pub document_size: i32,
    pub document_type: String,
    pub document_extension: String,
    pub document_permissions: i32,
    pub document_md5_hash: String,
    pub document_ssdeep_hash: String,
    pub entity_data: String,
    pub entity_keywords: Vec<String>,
    pub highlight: Option<HighlightEntity>,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_created: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_modified: Option<DateTime<Utc>>,
}

impl Document {
    pub fn append_highlight(&mut self, highlight: Option<HighlightEntity>) {
        self.highlight = highlight
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct HighlightEntity {
    pub entity_data: Vec<String>,
}

impl From<FileData> for Document {
    fn from(value: FileData) -> Self {
        DocumentBuilder::default()
            .bucket_uuid(value.bucket_uuid)
            .bucket_path(value.bucket_path)
            .document_name(value.document_name)
            .document_path(value.document_path)
            .document_size(value.document_size)
            .document_type(value.document_type)
            .document_extension(value.document_extension)
            .document_permissions(value.document_permissions)
            .document_md5_hash(value.document_md5_hash)
            .document_ssdeep_hash(value.document_ssdeep_hash)
            .entity_data(value.entity_data)
            .entity_keywords(value.entity_keywords)
            .highlight(Option::<HighlightEntity>::None)
            .document_created(value.document_created)
            .document_modified(value.document_modified)
            .build()
            .unwrap()
    }
}
