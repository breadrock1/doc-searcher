use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Clone, Default, Getters, Deserialize, Serialize, ToSchema)]
pub struct Cluster {
    #[schema(example = "172.19.0.2")]
    ip: String,
    #[getset(get = "pub")]
    #[schema(example = "d93df49fa6ff")]
    name: String,
    #[schema(example = "cdfhilmrstw")]
    #[serde(alias = "node.role")]
    node_role: String,
    #[schema(example = "*")]
    master: String,
    #[schema(example = "32")]
    #[serde(alias = "heap.percent")]
    heap_percent: String,
    #[schema(example = "67")]
    #[serde(alias = "ram.percent")]
    ram_percent: String,
    #[schema(example = "2")]
    cpu: String,
}

impl Cluster {
    pub fn builder() -> ClusterBuilder {
        ClusterBuilder::default()
    }
}
