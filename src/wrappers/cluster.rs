use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, Builder, Default, Clone, ToSchema)]
pub struct Cluster {
    pub ip: String,
    #[serde(alias = "heap.percent")]
    pub heap_percent: String,
    #[serde(alias = "ram.percent")]
    pub ram_percent: String,
    pub cpu: String,
    pub load_1m: String,
    pub load_5m: String,
    pub load_15m: String,
    #[serde(alias = "node.role")]
    pub node_role: String,
    pub master: String,
    pub name: String,
}

#[derive(Deserialize, IntoParams)]
pub struct ClusterForm {
    cluster_name: String,
}

impl Display for ClusterForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_data = &self.cluster_name;
        write!(f, "{}", self_data.clone())
    }
}
