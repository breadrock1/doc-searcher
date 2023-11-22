use elasticsearch::Elasticsearch;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct ElasticContext {
    context: Arc<RwLock<Elasticsearch>>,
}

impl ElasticContext {
    pub fn _new(elastic: Elasticsearch) -> Self {
        let context = Arc::new(RwLock::new(elastic));
        ElasticContext { context }
    }

    pub fn get_cxt(&self) -> &Arc<RwLock<Elasticsearch>> {
        &self.context
    }
}
