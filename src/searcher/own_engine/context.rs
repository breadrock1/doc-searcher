use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct OtherContext {
    context: Arc<RwLock<String>>,
}

impl OtherContext {
    pub fn _new(elastic: String) -> Self {
        let elastic = Arc::new(RwLock::new(elastic));
        OtherContext { context: elastic }
    }

    pub fn get_cxt(&self) -> &Arc<RwLock<String>> {
        &self.context
    }
}
