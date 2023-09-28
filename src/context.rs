use std::sync::{Arc, Mutex};
use elasticsearch::Elasticsearch;
use elasticsearch::http::transport::Transport;

#[derive(Default, Clone)]
pub struct SearchContext {
    context: Arc<Mutex<Elasticsearch>>,
}

impl SearchContext {
    pub fn _new(url: &str) -> Option<Self> {
        let trans = Transport::single_node(url).unwrap();
        let elastic = Elasticsearch::new(trans);
        let elastic_ref = Arc::new(Mutex::new(elastic));
        Some(SearchContext { context: elastic_ref })
    }

    pub fn get_cxt(&self) -> &Arc<Mutex<Elasticsearch>> {
        &self.context
    }
}

