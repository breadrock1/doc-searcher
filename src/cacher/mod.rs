pub mod config;

#[cfg(feature = "enable-cacher-redis")]
pub mod redis;

#[async_trait::async_trait]
pub trait CacherService<K, V> {
    async fn insert(&self, key: &K, value: &V);

    async fn load(&self, key: &K) -> Option<V>;
}
