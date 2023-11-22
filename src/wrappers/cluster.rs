use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default)]
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

#[derive(Deserialize, Serialize)]
pub struct ClusterForm {
    cluster_name: String,
}

impl ToString for ClusterForm {
    fn to_string(&self) -> String {
        let self_data = &self.cluster_name;
        self_data.clone()
    }
}
