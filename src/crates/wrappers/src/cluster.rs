use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use std::fmt::Display;

#[derive(Serialize, Deserialize, Builder, Default, Clone, ToSchema)]
pub struct Cluster {
    #[schema(example = "172.19.0.2")]
    pub ip: String,
    #[schema(example = "32")]
    #[serde(alias = "heap.percent")]
    pub heap_percent: String,
    #[schema(example = "67")]
    #[serde(alias = "ram.percent")]
    pub ram_percent: String,
    #[schema(example = "0")]
    pub cpu: String,
    #[schema(example = "0.00")]
    pub load_1m: String,
    #[schema(example = "0.05")]
    pub load_5m: String,
    #[schema(example = "0.05")]
    pub load_15m: String,
    #[schema(example = "cdfhilmrstw")]
    #[serde(alias = "node.role")]
    pub node_role: String,
    #[schema(example = "*")]
    pub master: String,
    #[schema(example = "d93df49fa6ff")]
    pub name: String,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ClusterForm {
    cluster_name: String,
}

impl Display for ClusterForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_data = &self.cluster_name;
        write!(f, "{}", self_data.clone())
    }
}
