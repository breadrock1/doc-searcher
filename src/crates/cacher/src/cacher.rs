use crate::AnyCacherService;
use crate::values::VecCacherDocuments;
use crate::values::{CacherDocument, CacherSearchParams};

use redis::{AsyncCommands, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RedisService {
    client: Arc<RwLock<redis::Client>>,
}

impl RedisService {
    pub fn new(address: &str) -> Self {
        let redis_client = redis::Client::open(address);
        let client_arc = Arc::new(RwLock::new(redis_client.unwrap()));
        RedisService {
            client: client_arc,
        }
    }
}

impl Default for RedisService {
    fn default() -> Self {
        let address = "redis://127.0.0.1";
        let redis_client = redis::Client::open(address);
        let client_arc = Arc::new(RwLock::new(redis_client.unwrap()));
        RedisService {
            client: client_arc,
        }
    }
}

impl AnyCacherService for RedisService {
    async fn get_documents(&self, search_params: &CacherSearchParams) -> Option<Vec<CacherDocument>> {
        let cxt = self.client.read().await;
        let mut conn = cxt.get_tokio_connection().await.unwrap();
        match conn.get(search_params.query.as_str()).await.unwrap() {
            Value::Nil => None,
            Value::Data(value) => {
                let docs: Vec<CacherDocument> = serde_json::from_slice(value.as_slice()).unwrap();
                Some(docs)
            },
            _ => None,
        }
    }

    async fn set_documents(&self, params: &CacherSearchParams, docs: Vec<CacherDocument>) {
        let cxt = self.client.read().await;
        let mut conn = cxt.get_async_connection().await.unwrap();
        let vec_docs = VecCacherDocuments::from(docs);
        let vec_docs_str = serde_json::to_string(&vec_docs).unwrap();

        let _: ((), i64, usize) = redis::pipe()
            .cmd("SET")
            .arg(params.query.as_str())
            .arg(vec_docs_str.as_str())
            .query_async(&mut conn)
            .await
            .unwrap();
    }
}
