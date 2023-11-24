use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Serialize, Default)]
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
        skip_serializing_if = "Option::is_none"
    )]
    pub document_created: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_modified: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct HighlightEntity {
    pub entity_data: Vec<String>,
}

impl HighlightEntity {
    pub fn create(entity_data: Vec<String>) -> Self {
        HighlightEntity { entity_data }
    }
}

impl Document {
    pub fn create(
        bucket_uuid: String,
        bucket_path: String,
        document_name: String,
        document_path: String,
        document_size: i32,
        document_type: String,
        document_extension: String,
        document_permissions: i32,
        document_md5_hash: String,
        document_ssdeep_hash: String,
        entity_data: String,
        entity_keywords: Vec<String>,
        highlight: Option<HighlightEntity>,
        document_created: Option<DateTime<Utc>>,
        document_modified: Option<DateTime<Utc>>,
    ) -> Self {
        Document {
            bucket_uuid,
            bucket_path,
            document_name,
            document_path,
            document_size,
            document_type,
            document_extension,
            document_permissions,
            document_md5_hash,
            document_ssdeep_hash,
            entity_data,
            entity_keywords,
            highlight,
            document_created,
            document_modified,
        }
    }

    pub fn append_highlight(&mut self, highlight: Option<HighlightEntity>) {
        self.highlight = highlight
    }
}

pub fn serialize_dt<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(dt) = dt {
        dt.format("%Y-%m-%dT%H:%M:%SZ")
            .to_string()
            .serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}
