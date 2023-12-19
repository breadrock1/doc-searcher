use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Default)]
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

impl Display for ClusterForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_data = &self.cluster_name;
        write!(f, "{}", self_data.clone())
    }
}
