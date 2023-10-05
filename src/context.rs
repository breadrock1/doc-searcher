use elasticsearch::Elasticsearch;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct SearchContext {
    context: Arc<Mutex<Elasticsearch>>,
}

impl SearchContext {
    pub fn _new(elastic: Elasticsearch) -> Option<Self> {
        let elastic = Arc::new(Mutex::new(elastic));
        Some(SearchContext { context: elastic })
    }

    pub fn get_cxt(&self) -> &Arc<Mutex<Elasticsearch>> {
        &self.context
    }
}
