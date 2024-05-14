use crate::forms::cluster::Cluster;
use crate::forms::document::Document;
use crate::forms::folder::Folder;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct OtherContext {
    context: Arc<RwLock<SearchEngine>>,
}

impl OtherContext {
    pub fn new(_: String) -> Self {
        let engine = SearchEngine::default();
        OtherContext {
            context: Arc::new(RwLock::new(engine)),
        }
    }

    pub fn get_cxt(&self) -> &Arc<RwLock<SearchEngine>> {
        &self.context
    }
}

pub struct SearchEngine {
    pub buckets: Arc<RwLock<HashMap<String, Folder>>>,
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
