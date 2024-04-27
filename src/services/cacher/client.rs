use crate::services::CacherService;

use redis::RedisResult;
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct RedisService {
    cacher_client: Arc<RwLock<redis::Client>>,
    expire_time: u64,
}

impl RedisService {
    pub fn new(address: &str, expire: u64) -> Self {
        let redis_client = redis::Client::open(address);
        let client_arc = Arc::new(RwLock::new(redis_client.unwrap()));
        RedisService {
            cacher_client: client_arc,
            expire_time: expire,
        }
    }

    pub fn get_cache_ref(&self) -> Arc<RwLock<redis::Client>> {
        self.cacher_client.clone()
    }

    pub fn get_expire(&self) -> u64 {
        self.expire_time
    }
}

impl Default for RedisService {
    fn default() -> Self {
        let address = "redis://127.0.0.1:6379/";
        let redis_client = redis::Client::open(address);
        let client_arc = Arc::new(RwLock::new(redis_client.unwrap()));
        RedisService {
            cacher_client: client_arc,
            expire_time: 3600,
        }
    }
}

#[async_trait::async_trait]
impl CacherService for RedisService {
    async fn insert<T, U>(&self, key: T, value: U) -> U
    where
        T: ToRedisArgs + Send + Sync,
        U: ToRedisArgs + Send + Sync,
    {
        let cxt = self.cacher_client.read().await;
        let connection_res = cxt.get_multiplexed_tokio_connection().await;
        if connection_res.is_err() {
            let err = connection_res.err().unwrap();
            log::warn!("Failed to get redis service connection {}", err);
            return value;
        }

        let mut connection = connection_res.unwrap();
        let _: RedisResult<redis::Value> = connection.set_ex(&key, &value, self.get_expire()).await;
        value
    }

    async fn load<T, U>(&self, key: T) -> Option<U>
    where
        T: ToRedisArgs + Send + Sync,
        U: FromRedisValue + Send + Sync,
    {
        let cxt = self.cacher_client.read().await;
        let connection_res = cxt.get_multiplexed_tokio_connection().await;
        if connection_res.is_err() {
            let err = connection_res.err().unwrap();
            log::warn!("Failed to get redis service connection {}", err);
            return None;
        }

        let mut connection = connection_res.unwrap();
        connection.get(&key).await.ok()
    }
}
