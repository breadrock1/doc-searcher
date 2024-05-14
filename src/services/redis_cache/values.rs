use crate::forms::document::Document;
use crate::forms::s_params::SearchParams;

use redis::{ErrorKind, RedisError, RedisResult, Value};
use redis::{FromRedisValue, RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct CacherSearchParams {
    search_params: SearchParams,
}

impl From<&SearchParams> for CacherSearchParams {
    fn from(value: &SearchParams) -> Self {
        CacherSearchParams {
            search_params: value.to_owned(),
        }
    }
}

impl From<CacherSearchParams> for SearchParams {
    fn from(value: CacherSearchParams) -> SearchParams {
        value.search_params
    }
}

impl ToRedisArgs for CacherSearchParams {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let json_str = serde_json::to_string(&self.search_params).unwrap();
        out.write_arg_fmt(json_str)
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CacherDocuments {
    documents: Vec<Document>,
}

impl From<&Vec<Document>> for CacherDocuments {
    fn from(value: &Vec<Document>) -> Self {
        CacherDocuments {
            documents: value.to_owned(),
        }
    }
}

impl From<CacherDocuments> for Vec<Document> {
    fn from(value: CacherDocuments) -> Vec<Document> {
        value.documents
    }
}

impl ToRedisArgs for CacherDocuments {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let json_str = serde_json::to_string(self).unwrap();
        out.write_arg_fmt(json_str)
    }
}

impl FromRedisValue for CacherDocuments {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::Data(data) => serde_json::from_slice::<CacherDocuments>(data.as_slice())
                .map_err(|_| {
                    let msg = "Failed while deserializing document from redis_cache";
                    RedisError::from((ErrorKind::IoError, msg))
                }),
            _ => {
                let err = anyhow::Error::msg("Incorrect redis_cache value type to deserialize");
                let io_err = std::io::Error::new(std::io::ErrorKind::InvalidData, err);
                Err(io_err.into())
            }
        }
    }
}
