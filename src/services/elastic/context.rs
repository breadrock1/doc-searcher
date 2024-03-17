use elasticsearch::Elasticsearch;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct ElasticContext {
    context: Arc<RwLock<Elasticsearch>>,
}

impl ElasticContext {
    pub fn new(elastic: Elasticsearch) -> Self {
        ElasticContext {
            context: Arc::new(RwLock::new(elastic)),
        }
    }

    pub fn get_cxt(&self) -> &Arc<RwLock<Elasticsearch>> {
        &self.context
    }
}
