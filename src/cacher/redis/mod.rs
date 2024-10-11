use crate::cacher::config::CacherConfig;
use crate::cacher::CacherService;
use crate::Connectable;

use getset::CopyGetters;
use redis::{AsyncCommands, Client, RedisError, RedisResult};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, CopyGetters)]
pub struct RedisClient {
    // #[getset(get_copy = "pub")]
    options: Arc<RedisOptions>,
    // #[getset(get_copy = "pub")]
    client: Arc<RwLock<Client>>,
}

impl RedisClient {
    pub fn options(&self) -> Arc<RedisOptions> {
        self.options.clone()
    }

    pub fn client(&self) -> Arc<RwLock<Client>> {
        self.client.clone()
    }
}

#[derive(CopyGetters)]
pub struct RedisOptions {
    #[getset(get_copy = "pub")]
    expire: u64
}

impl From<&CacherConfig> for RedisOptions {
    fn from(value: &CacherConfig) -> Self {
        RedisOptions {
            expire: value.expired(),
        }
    }
}

impl Connectable for RedisClient {
    type Config = CacherConfig;
    type Error = RedisError;
    type Service = RedisClient;

    fn connect(config: &Self::Config) -> Result<Self::Service, Self::Error> {
        let address = format!("redis://{}/", config.address());
        let client = Client::open(address)?;
        let options = RedisOptions::from(config);
        Ok(RedisClient {
            options: Arc::new(options),
            client: Arc::new(RwLock::new(client)),
        })
    }
}

#[async_trait::async_trait]
impl <K, V> CacherService<K, V> for RedisClient
where
    K: redis::ToRedisArgs + Sync,
    V: redis::ToRedisArgs + redis::FromRedisValue + Sync
{
    async fn insert(&self, key: &K, value: &V) {
        let expired_secs = self.options().expire();
        let cxt = self.client.read().await;
        match cxt.get_multiplexed_tokio_connection().await {
            Err(err) => {
                tracing::warn!("Failed to get redis service connection {err:#?}");
                return;
            }
            Ok(mut conn) => {
                let _: RedisResult<redis::Value> = conn.set_ex(key, value, expired_secs).await;
            }
        }
    }

    async fn load(&self, key: &K) -> Option<V> {
        let cxt = self.client.read().await;
        match cxt.get_multiplexed_tokio_connection().await {
            Ok(mut conn) => conn.get(key).await.ok(),
            Err(err) => {
                tracing::warn!("Failed to get redis service connection {err:#?}");
                None
            }
        }
    }
}
