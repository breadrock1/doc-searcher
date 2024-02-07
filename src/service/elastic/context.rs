use cacher::cacher::RedisService;
use elasticsearch::Elasticsearch;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct ElasticContext {
    context: Arc<RwLock<Elasticsearch>>,
    cacher: Arc<RwLock<RedisService>>,
}

impl ElasticContext {
    pub fn _new(elastic: Elasticsearch) -> Self {
        let cache_service = RedisService::default();
        ElasticContext {
            context: Arc::new(RwLock::new(elastic)),
            cacher: Arc::new(RwLock::new(cache_service)),
        }
    }

    pub fn get_cxt(&self) -> &Arc<RwLock<Elasticsearch>> {
        &self.context
    }

    pub fn get_cacher(&self) -> &Arc<RwLock<RedisService>> {
        &self.cacher
    }
}
