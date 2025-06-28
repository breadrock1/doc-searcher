#[async_trait::async_trait]
pub trait Cacher<K, V> {
    async fn store(&self, key: &K, value: &V);
    async fn load(&self, key: &K) -> Option<V>;
}
