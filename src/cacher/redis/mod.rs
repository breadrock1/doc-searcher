pub mod config;
mod models;

use crate::cacher::redis::config::RedisConfig;
use crate::cacher::CacherService;
use crate::ServiceConnect;

use getset::CopyGetters;
use redis::{AsyncCommands, Client, RedisError, RedisResult};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, CopyGetters)]
pub struct RedisClient {
    options: Arc<RedisConfig>,
    client: Arc<RwLock<Client>>,
}

#[async_trait::async_trait]
impl ServiceConnect for RedisClient {
    type Config = RedisConfig;
    type Error = RedisError;
    type Client = RedisClient;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let address = config.address().as_str();
        let client = Client::open(address)?;
        Ok(RedisClient {
            options: Arc::new(config.to_owned()),
            client: Arc::new(RwLock::new(client)),
        })
    }
}

#[async_trait::async_trait]
impl<K, V> CacherService<K, V> for RedisClient
where
    K: redis::ToRedisArgs + Sync,
    V: redis::ToRedisArgs + redis::FromRedisValue + Sync,
{
    async fn insert(&self, key: &K, value: &V) {
        let expired_secs = self.options.expired();
        let cxt = self.client.write().await;
        match cxt.get_multiplexed_tokio_connection().await {
            Err(err) => {
                tracing::warn!(err=?err, "failed to get redis service connection");
                return;
            }
            Ok(mut conn) => {
                let set_result: RedisResult<()> = conn.set_ex(key, value, expired_secs).await;
                if let Err(err) = set_result {
                    tracing::error!(err=?err, "failed to insert value to redis");
                    return;
                };
            }
        }
    }

    async fn load(&self, key: &K) -> Option<V> {
        let cxt = self.client.read().await;
        match cxt.get_multiplexed_tokio_connection().await {
            Ok(mut conn) => conn.get(key).await.ok(),
            Err(err) => {
                tracing::warn!(err=?err, "failed to get redis service connection");
                None
            }
        }
    }
}
