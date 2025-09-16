pub mod config;
mod dto;

use redis::{AsyncCommands, Client};
use redis::{FromRedisValue, ToRedisArgs};
use redis::{RedisError, RedisResult};
use std::sync::Arc;
use tokio::sync::RwLock;

use self::config::RedisConfig;

use crate::application::services::cacher::Cacher;
use crate::ServiceConnect;

#[derive(Clone)]
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
        tracing::debug!(url=address, "connected to redis");
        Ok(RedisClient {
            options: Arc::new(config.to_owned()),
            client: Arc::new(RwLock::new(client)),
        })
    }
}

#[async_trait::async_trait]
impl<K, V> Cacher<K, V> for RedisClient
where
    K: ToRedisArgs + Sync,
    V: ToRedisArgs + FromRedisValue + Sync,
{
    async fn store(&self, key: &K, value: &V)
    where
        K: ToRedisArgs,
        V: ToRedisArgs + FromRedisValue,
    {
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

    async fn load(&self, key: &K) -> Option<V>
    where
        K: ToRedisArgs + Sync,
        V: ToRedisArgs + FromRedisValue + Sync,
    {
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
