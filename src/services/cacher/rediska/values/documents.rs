use crate::forms::documents::document::Document;
use crate::forms::documents::similar::DocumentSimilar;

use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult};
use redis::{RedisWrite, ToRedisArgs, Value};
use serde_derive::{Deserialize, Serialize};

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

impl From<&Vec<DocumentSimilar>> for CacherDocuments {
    fn from(value: &Vec<DocumentSimilar>) -> Self {
        let documents = value
            .into_iter()
            .map(DocumentSimilar::get_document)
            .collect::<Vec<Document>>();

        CacherDocuments { documents }
    }
}

impl From<CacherDocuments> for Vec<Document> {
    fn from(value: CacherDocuments) -> Vec<Document> {
        value.documents
    }
}

impl From<CacherDocuments> for Vec<DocumentSimilar> {
    fn from(value: CacherDocuments) -> Self {
        value
            .documents
            .into_iter()
            .map(DocumentSimilar::from)
            .collect::<Vec<DocumentSimilar>>()
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
                    let msg = "Failed while deserializing document from redis";
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
