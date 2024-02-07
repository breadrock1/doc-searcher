use chrono::{DateTime, Utc};
use derive_builder::Builder;
use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize)]
pub struct CacherDocument {
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

impl ToRedisArgs for &CacherDocument {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite
    {
        let json_str = serde_json::to_string(self).unwrap();
        out.write_arg_fmt(json_str)
    }
}

#[derive(Deserialize, Serialize)]
pub struct VecCacherDocuments {
    pub docs: Vec<CacherDocument>,
}

impl From<Vec<CacherDocument>> for VecCacherDocuments {
    fn from(value: Vec<CacherDocument>) -> Self {
        VecCacherDocuments {
            docs: value
        }
    }
}

impl ToRedisArgs for VecCacherDocuments {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite
    {
        let json_str = serde_json::to_string(self).unwrap();
        out.write_arg_fmt(json_str)
    }
}

#[derive(Builder, Serialize, Deserialize, Clone, Default)]
pub struct CacherSearchParams {
    pub query: String,
    pub document_type: String,
    pub document_extension: String,
    pub document_size_to: i64,
    pub document_size_from: i64,
    pub created_date_to: String,
    pub created_date_from: String,
    pub result_size: i64,
    pub result_offset: i64,
}

impl ToRedisArgs for &CacherSearchParams {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite
    {
        let json_str = serde_json::to_string(self).unwrap();
        out.write_arg_fmt(json_str)
    }
}
