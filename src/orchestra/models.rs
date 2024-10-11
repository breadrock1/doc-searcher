use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Clone, Default, Getters, Deserialize, Serialize, ToSchema)]
pub struct Cluster {
    #[schema(example = "172.19.0.2")]
    ip: String,
    #[schema(example = "32")]
    #[serde(alias = "heap.percent")]
    heap_percent: String,
    #[schema(example = "67")]
    #[serde(alias = "ram.percent")]
    ram_percent: String,
    #[schema(example = "2")]
    cpu: String,
    #[schema(example = "0.00")]
    load_1m: String,
    #[schema(example = "0.05")]
    load_5m: String,
    #[schema(example = "0.05")]
    load_15m: String,
    #[schema(example = "cdfhilmrstw")]
    #[serde(alias = "node.role")]
    node_role: String,
    #[schema(example = "*")]
    master: String,
    #[getset(get = "pub")]
    #[schema(example = "d93df49fa6ff")]
    name: String,
}

impl Cluster {
    pub fn builder() -> ClusterBuilder {
        ClusterBuilder::default()
    }
}
