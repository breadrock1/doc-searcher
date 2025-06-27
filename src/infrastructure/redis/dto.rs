use redis::{RedisError, RedisResult, RedisWrite, Value};
use serde::ser::Error;

use crate::application::dto::{
    Document,
    Paginated,
    FullTextSearchParams, SemanticSearchParams,
};

impl redis::ToRedisArgs for FullTextSearchParams {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match serde_json::to_string(self) {
            Ok(json_str) => out.write_arg_fmt(json_str),
            Err(err) => {
                tracing::error!(err=?err, "cacher: failed to serialize paginate form");
            }
        }
    }
}

impl redis::ToRedisArgs for SemanticSearchParams {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match serde_json::to_string(self) {
            Ok(json_str) => out.write_arg_fmt(json_str),
            Err(err) => {
                tracing::error!(err=?err, "cacher: failed to serialize search parameters");
            }
        }
    }
}

impl redis::ToRedisArgs for Paginated<Vec<Document>> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match serde_json::to_string(self) {
            Ok(json_str) => out.write_arg_fmt(json_str),
            Err(err) => {
                tracing::error!(err=?err, "cacher: failed to serialize paginated docs");
            }
        }
    }
}

impl redis::FromRedisValue for Paginated<Vec<Document>> {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::BulkString(data) => {
                serde_json::from_slice::<Paginated<Vec<Document>>>(data).map_err(RedisError::from)
            }
            _ => {
                let err = serde_json::Error::custom("failed to extract redis value type");
                Err(RedisError::from(err))
            }
        }
    }
}
