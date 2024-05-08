use crate::services::init::ServiceParameters;

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
    pub fn new(elastic: Elasticsearch, service_params: &ServiceParameters) -> Self {
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
    logger_service_host: String,
    cacher_service_host: String,
    cacher_expire: u64,
    ocr_service_host: String,
    watcher_service_host: String,
    llm_service_host: String,
    global_folders: String,
}

impl Default for ContextOptions {
    fn default() -> Self {
        ContextOptions {
            logger_service_host: "http://localhost:4444".into(),
            cacher_service_host: "redis://localhost:6379".into(),
            cacher_expire: 3600u64,
            ocr_service_host: "http://localhost:8083".into(),
            watcher_service_host: "http://localhost:2893".into(),
            llm_service_host: "http://localhost:8085".into(),
            global_folders: "common_folder".into(),
        }
    }
}

impl From<&ServiceParameters> for ContextOptions {
    fn from(value: &ServiceParameters) -> Self {
        ContextOptions {
            logger_service_host: value.logger_service_host().into(),
            cacher_service_host: value.cacher_service_host().into(),
            cacher_expire: *value.cacher_expire(),
            ocr_service_host: value.ocr_service_host().into(),
            watcher_service_host: value.watcher_service_host().into(),
            llm_service_host: value.llm_service_host().into(),
            global_folders: value.global_folders().into(),
        }
    }
}
