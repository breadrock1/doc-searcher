use crate::values::{MaybeSearchParams, VecCacherDocuments};
use crate::AnyCacherService;

use wrappers::search_params::SearchParams;

use redis::RedisResult;
use redis::{AsyncCommands, FromRedisValue};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RedisService {
    client: Arc<RwLock<redis::Client>>,
}

impl RedisService {
    pub fn new(address: &str) -> Self {
        let redis_client = redis::Client::open(address);
        let client_arc = Arc::new(RwLock::new(redis_client.unwrap()));
        RedisService { client: client_arc }
    }
}

impl Default for RedisService {
    fn default() -> Self {
        let address = "redis://127.0.0.1:6379/";
        let redis_client = redis::Client::open(address);
        let client_arc = Arc::new(RwLock::new(redis_client.unwrap()));
        RedisService { client: client_arc }
    }
}

#[async_trait::async_trait]
impl AnyCacherService for RedisService {
    async fn get_documents(&self, search_params: &SearchParams) -> Option<VecCacherDocuments> {
        let cxt = self.client.read().await;
        let conn_result = cxt.get_tokio_connection().await;
        if conn_result.is_err() {
            let err = conn_result.err().unwrap();
            println!("{:?}", err);
            return None;
        }

        let mut conn = conn_result.unwrap();
        match conn.get(search_params.query.as_str()).await {
            Ok(redis_value) => VecCacherDocuments::from_redis_value(&redis_value).ok(),
            Err(err) => {
                println!("Failed parsing RedisValue object: {}", err);
                return None;
            }
        }
    }

    async fn set_documents(
        &self,
        params: &SearchParams,
        docs: VecCacherDocuments,
    ) -> VecCacherDocuments {
        let cxt = self.client.read().await;
        let conn_result = cxt.get_tokio_connection().await;
        if conn_result.is_err() {
            let err = conn_result.err().unwrap();
            println!("{:?}", err);
            return docs;
        }

        let mut conn = conn_result.unwrap();
        let maybe_search_params = MaybeSearchParams::from(params);
        let set_result: RedisResult<()> = conn.set_ex(&maybe_search_params, &docs, 3600).await;
        println!("{:?}", set_result);
        docs
    }
}
