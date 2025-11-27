use doc_search_core::ServiceConnect;
use redis::{AsyncCommands, Client, RedisError, RedisResult};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::config::RedisConfig;

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
        tracing::debug!(url = address, "connected to redis");
        Ok(RedisClient {
            options: Arc::new(config.to_owned()),
            client: Arc::new(RwLock::new(client)),
        })
    }
}

impl RedisClient {
    pub(crate) async fn store(&self, key: &String, value: Vec<u8>) {
        let expired_secs = self.options.expired();
        let cxt = self.client.write().await;
        match cxt.get_multiplexed_tokio_connection().await {
            Err(err) => {
                tracing::warn!(err=?err, "failed to get redis service connection");
            }
            Ok(mut conn) => {
                let set_result: RedisResult<()> = conn.set_ex(key, value, expired_secs).await;
                if let Err(err) = set_result {
                    tracing::warn!(err=?err, "failed to insert value to redis");
                };
            }
        }
    }

    pub(crate) async fn load(&self, key: &String) -> Option<Vec<u8>> {
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
