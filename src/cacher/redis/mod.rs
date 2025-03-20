mod models;
mod config;

use crate::cacher::config::CacherConfig;
use crate::cacher::CacherService;
use crate::searcher::forms::{FulltextParams, ScrollNextForm, SemanticParams};
use crate::searcher::models::Paginated;
use crate::ServiceConnect;

use getset::CopyGetters;
use redis::{AsyncCommands, Client, RedisError, RedisResult};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type FullTextParamsCached = Box<dyn CacherService<FulltextParams, Paginated<Vec<Value>>>>;
pub type SemanticParamsCached = Box<dyn CacherService<SemanticParams, Paginated<Vec<Value>>>>;
pub type PaginatedCached = Box<dyn CacherService<ScrollNextForm, Paginated<Vec<Value>>>>;

#[derive(Clone, CopyGetters)]
pub struct RedisClient {
    options: Arc<RedisConfig>,
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
    expire: u64,
}

impl From<&CacherConfig> for RedisOptions {
    fn from(value: &CacherConfig) -> Self {
        RedisOptions {
            expire: value.expired(),
        }
    }
}

impl ServiceConnect for RedisClient {
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
impl<K, V> CacherService<K, V> for RedisClient
where
    K: redis::ToRedisArgs + Sync,
    V: redis::ToRedisArgs + redis::FromRedisValue + Sync,
{
    async fn insert(&self, key: &K, value: &V) {
        let expired_secs = self.options().expire();
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
