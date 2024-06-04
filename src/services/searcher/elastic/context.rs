use crate::services::config::ServiceConfig;

use derive_getters::Getters;
use elasticsearch::Elasticsearch;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct ElasticContext {
    context: Arc<RwLock<Elasticsearch>>,
    options: Arc<ContextOptions>,
}

impl ElasticContext {
    pub fn new(elastic: Elasticsearch, service_params: &ServiceConfig) -> Self {
        let options = ContextOptions::from(service_params);
        ElasticContext {
            context: Arc::new(RwLock::new(elastic)),
            options: Arc::new(options),
        }
    }
    pub fn get_cxt(&self) -> &Arc<RwLock<Elasticsearch>> {
        &self.context
    }
    pub fn get_options(&self) -> &Arc<ContextOptions> {
        &self.options
    }
}

#[derive(Getters)]
pub struct ContextOptions {
    cacher_address: String,
    cacher_expire: u64,
    llm_address: String,
    logger_address: String,
    watcher_address: String,
}

impl Default for ContextOptions {
    fn default() -> Self {
        ContextOptions {
            cacher_expire: 3600u64,
            cacher_address: "redis://localhost:6379".into(),
            llm_address: "http://localhost:8085".into(),
            logger_address: "http://localhost:4444".into(),
            watcher_address: "http://localhost:2893".into(),
        }
    }
}

impl From<&ServiceConfig> for ContextOptions {
    fn from(value: &ServiceConfig) -> Self {
        ContextOptions {
            cacher_address: value.cacher_host().into(),
            cacher_expire: *value.cacher_expire(),
            llm_address: value.llm_host().into(),
            logger_address: value.logger_host().into(),
            watcher_address: value.watcher_host().into(),
        }
    }
}
