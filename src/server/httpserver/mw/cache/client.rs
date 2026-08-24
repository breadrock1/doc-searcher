use doc_search_core::ServiceConnect;
use redis::{AsyncCommands, Client, RedisError, RedisResult};
use std::sync::Arc;

use super::config::RedisConfig;

#[async_trait::async_trait]
pub trait CacheRepository: Send + Sync {
    async fn set_ex(&self, key: &str, value: Vec<u8>, ttl: u64) -> RedisResult<()>;
    async fn get(&self, key: &str) -> RedisResult<Vec<u8>>;
}

struct RedisRepository {
    client: Client,
}

#[async_trait::async_trait]
impl CacheRepository for RedisRepository {
    async fn set_ex(&self, key: &str, value: Vec<u8>, ttl: u64) -> RedisResult<()> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await?;
        conn.set_ex(key, value, ttl).await
    }

    async fn get(&self, key: &str) -> RedisResult<Vec<u8>> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await?;
        conn.get(key).await
    }
}

#[derive(Clone)]
pub struct RedisClient {
    options: Arc<RedisConfig>,
    repository: Arc<dyn CacheRepository>,
}

#[async_trait::async_trait]
impl ServiceConnect for RedisClient {
    type Config = RedisConfig;
    type Client = RedisClient;
    type Error = RedisError;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let address = config.address().as_str();
        let client = Client::open(address)?;
        tracing::debug!(url = address, "connected to redis");
        Ok(RedisClient::new(
            config.to_owned(),
            Arc::new(RedisRepository { client }),
        ))
    }
}

impl RedisClient {
    fn new(options: RedisConfig, repository: Arc<dyn CacheRepository>) -> Self {
        RedisClient {
            options: Arc::new(options),
            repository,
        }
    }

    pub(crate) async fn store(&self, key: &str, value: Vec<u8>) {
        let expired_secs = self.options.expired();
        if let Err(err) = self.repository.set_ex(key, value, expired_secs).await {
            tracing::warn!(err=?err, "failed to insert value to redis");
        }
    }

    pub(crate) async fn load(&self, key: &str) -> Option<Vec<u8>> {
        self.repository.get(key).await.ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        Redis {}

        impl Clone for Redis {
            fn clone(&self) -> Self;
        }

        #[async_trait::async_trait]
        impl CacheRepository for Redis {
            async fn set_ex(&self, key: &str, value: Vec<u8>, ttl: u64) -> RedisResult<()>;
            async fn get(&self, key: &str) -> RedisResult<Vec<u8>>;
        }
    }

    fn redis_config() -> RedisConfig {
        serde_json::from_value(serde_json::json!({
            "address": "redis://localhost:6379",
            "username": "user",
            "password": "pass",
            "expired": 60,
        }))
        .expect("failed to deserialize redis config")
    }

    fn build_client(mock: MockRedis) -> RedisClient {
        RedisClient::new(redis_config(), Arc::new(mock))
    }

    fn redis_error() -> RedisError {
        RedisError::from(std::io::Error::other("boom"))
    }

    #[tokio::test]
    async fn test_store_success() {
        let mut mock = MockRedis::new();
        mock.expect_set_ex()
            .times(1)
            .withf(|_, _, ttl| *ttl == 60)
            .returning(|_, _, _| Ok(()));

        let client = build_client(mock);
        client.store("key", vec![1, 2, 3]).await;
    }

    #[tokio::test]
    async fn test_store_error_is_swallowed() {
        let mut mock = MockRedis::new();
        mock.expect_set_ex()
            .times(1)
            .returning(|_, _, _| Err(redis_error()));

        let client = build_client(mock);
        client.store("key", vec![1]).await;
    }

    #[tokio::test]
    async fn test_load_success() {
        let mut mock = MockRedis::new();
        mock.expect_get().times(1).returning(|_| Ok(vec![1, 2, 3]));

        let client = build_client(mock);
        let value = client.load("key").await;
        assert_eq!(Some(vec![1, 2, 3]), value);
    }

    #[tokio::test]
    async fn test_load_error_returns_none() {
        let mut mock = MockRedis::new();
        mock.expect_get().times(1).returning(|_| Err(redis_error()));

        let client = build_client(mock);
        let value = client.load("key").await;
        assert_eq!(None, value);
    }
}
