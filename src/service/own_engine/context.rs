use crate::wrappers::bucket::Bucket;
use crate::wrappers::cluster::Cluster;
use crate::wrappers::document::Document;

use std::collections::HashMap;
use std::sync::Arc;
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
}

impl OtherContext {
    pub fn _new(_: String) -> Self {
        let engine = SearchEngine::default();
        let elastic = Arc::new(RwLock::new(engine));
        OtherContext { context: elastic }
    }

    pub fn get_cxt(&self) -> &Arc<RwLock<SearchEngine>> {
        &self.context
    }
}
