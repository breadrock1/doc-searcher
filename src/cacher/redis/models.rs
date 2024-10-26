use crate::searcher::forms::{ScrollNextForm, SemanticParams};
use crate::searcher::models::Paginated;
use crate::storage::models::Document;

use redis::{RedisError, RedisResult, RedisWrite, Value};
use serde::ser::Error;

impl redis::ToRedisArgs for ScrollNextForm {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match serde_json::to_string(self) {
            Ok(json_str) => out.write_arg_fmt(json_str),
            Err(err) => {
                tracing::error!("cacher: failed to serialize paginate form: {err:#?}");
            }
        }
    }
}

impl redis::ToRedisArgs for SemanticParams {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match serde_json::to_string(self) {
            Ok(json_str) => out.write_arg_fmt(json_str),
            Err(err) => {
                tracing::error!("cacher: failed to serialize search parameters: {err:#?}");
            }
        }
    }
}

impl redis::ToRedisArgs for Document {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match serde_json::to_string(self) {
            Ok(json_str) => out.write_arg_fmt(json_str),
            Err(err) => {
                tracing::error!("cacher: failed to serialize document: {err:#?}");
            }
        }
    }
}

impl redis::ToRedisArgs for Paginated<Vec<serde_json::Value>> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        match serde_json::to_string(self) {
            Ok(json_str) => out.write_arg_fmt(json_str),
            Err(err) => {
                tracing::error!("cacher: failed to serialize paginated docs: {err:#?}");
            }
        }
    }
}

impl redis::FromRedisValue for Document {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::BulkString(data) => {
                serde_json::from_slice::<Document>(data.as_slice()).map_err(RedisError::from)
            }
            _ => {
                let err = serde_json::Error::custom("failed to extract redis value type");
                Err(RedisError::from(err))
            }
        }
    }
}

impl redis::FromRedisValue for Paginated<Vec<serde_json::Value>> {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::BulkString(data) => {
                serde_json::from_slice::<Paginated<Vec<serde_json::Value>>>(data.as_slice())
                    .map_err(RedisError::from)
            }
            _ => {
                let err = serde_json::Error::custom("failed to extract redis value type");
                Err(RedisError::from(err))
            }
        }
    }
}
