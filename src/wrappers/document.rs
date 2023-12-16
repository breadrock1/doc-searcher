use chrono::{DateTime, ParseResult, TimeZone, Utc};
use derive_builder::Builder;
use file_loader::FileData;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize, Serialize, Default, Builder)]
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

pub fn deserialize_dt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)
        .and_then(|value| Ok(format_datetime(value.as_str())))
        .and_then(|value| Ok(value.ok()))
}

fn format_datetime(value: &str) -> ParseResult<DateTime<Utc>> {
    Utc.datetime_from_str(value, "%Y-%m-%dT%H:%M:%SZ")
}

#[derive(Serialize, Deserialize, Default, Clone)]
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
