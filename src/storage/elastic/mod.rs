pub mod documents;
mod folders;

use elasticsearch::Elasticsearch;
use std::sync::Arc;
use tokio::sync::RwLock;

type EsCxt = Arc<RwLock<Elasticsearch>>;
