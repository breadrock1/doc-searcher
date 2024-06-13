use crate::forms::documents::embeddings::DocumentVectors;

use redis::{ErrorKind, RedisError, RedisResult};
use redis::{FromRedisValue, RedisWrite, ToRedisArgs, Value};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct CacherEmbeddings {
    embeddings: Vec<DocumentVectors>,
}

impl From<&Vec<DocumentVectors>> for CacherEmbeddings {
    fn from(value: &Vec<DocumentVectors>) -> Self {
        CacherEmbeddings {
            embeddings: value.to_owned(),
        }
    }
}

impl From<CacherEmbeddings> for Vec<DocumentVectors> {
    fn from(value: CacherEmbeddings) -> Vec<DocumentVectors> {
        value.embeddings
    }
}

impl ToRedisArgs for CacherEmbeddings {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let json_str = serde_json::to_string(self).unwrap();
        out.write_arg_fmt(json_str)
    }
}

impl FromRedisValue for CacherEmbeddings {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::Data(data) => serde_json::from_slice::<CacherEmbeddings>(data.as_slice())
                .map_err(|_| {
                    let msg = "Failed while deserializing embeddings from redis";
                    RedisError::from((ErrorKind::IoError, msg))
                }),
            _ => {
                let err = anyhow::Error::msg("Incorrect redis value type to deserialize");
                let io_err = std::io::Error::new(std::io::ErrorKind::InvalidData, err);
                Err(io_err.into())
            }
        }
    }
}
