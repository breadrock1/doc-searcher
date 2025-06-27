#[async_trait::async_trait]
pub trait Cacher<K, V>
where
    K: serde::Serialize,
    V: serde::Serialize + serde::de::DeserializeOwned,
{
    async fn store(&self, key: &K, value: &V);
    async fn load(&self, key: &K) -> Option<V>;
}
