use wrappers::bucket::Bucket;
use wrappers::cluster::Cluster;
use wrappers::document::Document;

use std::collections::HashMap;
use std::sync::Arc;
use cacher::cacher::RedisService;
use tokio::sync::RwLock;

pub struct SearchEngine {
    pub buckets: Arc<RwLock<HashMap<String, Bucket>>>,
    pub clusters: Arc<RwLock<HashMap<String, Cluster>>>,
    pub documents: Arc<RwLock<HashMap<String, Document>>>,
}

impl Default for SearchEngine {
    fn default() -> Self {
        SearchEngine {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            clusters: Arc::new(RwLock::new(HashMap::new())),
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Default, Clone)]
pub struct OtherContext {
    context: Arc<RwLock<SearchEngine>>,
    cacher: Arc<RwLock<RedisService>>,
}

impl OtherContext {
    pub fn _new(_: String) -> Self {
        let engine = SearchEngine::default();
        let cache_service = RedisService::default();
        OtherContext {
            context: Arc::new(RwLock::new(engine)),
            cacher: Arc::new(RwLock::new(cache_service)),
        }
    }

    pub fn get_cxt(&self) -> &Arc<RwLock<SearchEngine>> {
        &self.context
    }

    pub fn get_cacher(&self) -> &Arc<RwLock<RedisService>> {
        &self.cacher
    }
}
