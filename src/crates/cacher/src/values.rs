use wrappers::document::Document;
use wrappers::search_params::SearchParams;

use redis::{ErrorKind, RedisError, RedisResult, Value};
use redis::{FromRedisValue, RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};

pub struct MaybeSearchParams {
    pub search_params: Option<SearchParams>,
}

impl ToRedisArgs for &MaybeSearchParams {
    fn write_redis_args<W>(&self, out: &mut W)
        where
            W: ?Sized + RedisWrite
    {
        let json_str = serde_json::to_string(&self.search_params).unwrap();
        out.write_arg_fmt(json_str)
    }
}

impl From<&SearchParams> for MaybeSearchParams {
    fn from(value: &SearchParams) -> Self {
        MaybeSearchParams {
            search_params: Some(value.clone()),
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct VecCacherDocuments {
    pub cacher_documents: Vec<Document>,
}

impl VecCacherDocuments {
    pub fn get_documents(&self) -> &Vec<Document> {
        &self.cacher_documents
    }
}

impl From<Vec<Document>> for VecCacherDocuments {
    fn from(value: Vec<Document>) -> Self {
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
