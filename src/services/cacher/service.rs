use redis::{FromRedisValue, ToRedisArgs};

#[derive(Clone)]
pub struct CacherClient<D: CacherService> {
    pub service: D,
}

impl<D: CacherService> CacherClient<D> {
    pub fn new(service: D) -> Self {
        CacherClient { service }
    }
}

#[async_trait::async_trait]
pub trait CacherService {
    async fn insert<T, U>(&self, key: T, value: U) -> U
    where
        T: ToRedisArgs + Send + Sync,
        U: ToRedisArgs + Send + Sync;

    async fn load<T, U>(&self, key: T) -> Option<U>
    where
        T: ToRedisArgs + Send + Sync,
        U: FromRedisValue + Send + Sync;
}
