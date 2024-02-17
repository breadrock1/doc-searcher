use chrono::{DateTime, Utc};
use datetime::{deserialize_dt, serialize_dt};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, Clone, Default, IntoParams, ToSchema)]
pub struct HighlightEntity {
    pub entity_data: Vec<String>,
}

#[derive(Deserialize, Serialize, Builder, Default, Clone, ToSchema)]
pub struct Document {
    pub bucket_uuid: String,
    pub bucket_path: String,
    pub document_name: String,
    pub document_path: String,
    pub document_size: i32,
    pub document_type: String,
    pub document_extension: String,
    pub document_uuid: String,
    pub content_uuid: String,
    pub content: String,
    pub content_vector: Vec<String>,
    pub document_permissions: i32,
    pub document_md5_hash: String,
    pub document_ssdeep_hash: String,
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
    pub highlight: Option<HighlightEntity>,
}

impl Document {
    pub fn append_highlight(&mut self, highlight: Option<HighlightEntity>) {
        self.highlight = highlight
    }
}
