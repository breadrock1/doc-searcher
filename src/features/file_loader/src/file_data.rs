use chrono::DateTime;
use chrono::Utc;
use derive_builder::Builder;

#[derive(Builder)]
pub struct FileData {
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
    pub document_created: Option<DateTime<Utc>>,
    pub document_modified: Option<DateTime<Utc>>,
}
