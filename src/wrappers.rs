use std::os::linux::raw::stat;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DocumentJson {
    doc_id: u64,
    doc_name: String,
    doc_path: String,
    doc_ext: String,
}

impl DocumentJson {
    pub fn new(id: u64, name: &str, path: &str, ext: &str) -> Self {
        DocumentJson {
            doc_id: id,
            doc_name: name.to_string(),
            doc_path: path.to_string(),
            doc_ext: ext.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ClusterResult {
    result: u16,
}

impl ClusterResult {
    pub fn new(status: u16) -> Self {
        ClusterResult {
            result: status,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Cluster {
    ip: String,
    #[serde(alias = "heap.percent")]
    heap_percent: String,
    #[serde(alias = "ram.percent")]
    ram_percent: String,
    cpu: String,
    load_1m: String,
    load_5m: String,
    load_15m: String,
    #[serde(alias = "node.role")]
    node_role: String,
    master: String,
    name: String,
}


#[derive(Deserialize)]
pub struct ClusterForm {
    cluster_name: String,
}
