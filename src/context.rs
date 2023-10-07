use elasticsearch::Elasticsearch;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct SearchContext {
    context: Arc<RwLock<Elasticsearch>>,
}

impl SearchContext {
    pub fn _new(elastic: Elasticsearch) -> Self {
        let elastic = Arc::new(RwLock::new(elastic));
        SearchContext { context: elastic }
    }

    pub fn get_cxt(&self) -> &Arc<RwLock<Elasticsearch>> {
        &self.context
    }
}
