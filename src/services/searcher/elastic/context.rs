use crate::services::config::ServiceConfig;

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

pub struct ContextOptions {
    cacher_address: String,
    cacher_expire: u64,
    llm_address: String,
    logger_address: String,
}

impl ContextOptions {
    pub fn get_cacher_addr(&self) -> &str {
        self.cacher_address.as_str()
    }
    pub fn get_cacher_expire(&self) -> u64 {
        self.cacher_expire
    }
    pub fn get_llm_addr(&self) -> &str {
        self.llm_address.as_str()
    }
    pub fn get_logger_addr(&self) -> &str {
        self.logger_address.as_str()
    }
}

impl Default for ContextOptions {
    fn default() -> Self {
        ContextOptions {
            cacher_expire: 3600u64,
            cacher_address: "redis://localhost:6379".into(),
            llm_address: "http://localhost:8085".into(),
            logger_address: "http://localhost:4444".into(),
        }
    }
}

impl From<&ServiceConfig> for ContextOptions {
    fn from(value: &ServiceConfig) -> Self {
        ContextOptions {
            cacher_address: value.get_cacher_addr().into(),
            cacher_expire: value.get_cacher_expire(),
            llm_address: value.get_llm_addr().into(),
            logger_address: value.get_logger_addr().into(),
        }
    }
}
