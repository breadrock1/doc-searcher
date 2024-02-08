use chrono::{DateTime, Utc};
use derive_builder::Builder;
use redis::{ErrorKind, RedisError, RedisResult, Value};
use redis::{FromRedisValue, RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize)]
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

#[derive(Default, Deserialize, Serialize)]
pub struct VecCacherDocuments {
    pub cacher_documents: Vec<CacherDocument>,
}

impl VecCacherDocuments {
    pub fn get_documents(&self) -> &Vec<CacherDocument> {
        &self.cacher_documents
    }
}

impl From<Vec<CacherDocument>> for VecCacherDocuments {
    fn from(value: Vec<CacherDocument>) -> Self {
        VecCacherDocuments {
            cacher_documents: value
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

impl FromRedisValue for VecCacherDocuments {
    fn from_redis_value(redis_value: &Value) -> RedisResult<Self> {
        match redis_value {
            Value::Data(data) => {
                 serde_json::from_slice::<VecCacherDocuments>(data.as_slice())
                     .map_err(|_| {
                         let msg = "Faile while deserializing document from redis";
                         RedisError::from((ErrorKind::IoError, msg))
                     })
            },
            _ => {
                let err = anyhow::Error::msg("Incorrect redis value type to desrialize");
                let io_err = std::io::Error::new(std::io::ErrorKind::InvalidData, err);
                Err(io_err.into())
            }
        }
    }
}
