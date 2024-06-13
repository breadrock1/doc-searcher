use crate::forms::TestExample;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
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
    #[schema(example = "d93df49fa6ff")]
    name: String,
}

impl Cluster {
    pub fn builder() -> ClusterBuilder {
        ClusterBuilder::default()
    }

    pub fn get_ip(&self) -> &str {
        self.ip.as_str()
    }

    pub fn get_heap_percent(&self) -> &str {
        self.heap_percent.as_str()
    }

    pub fn get_ram_percent(&self) -> &str {
        self.ram_percent.as_str()
    }

    pub fn get_cpu(&self) -> &str {
        self.cpu.as_str()
    }

    pub fn get_load_1m(&self) -> &str {
        self.load_1m.as_str()
    }

    pub fn get_load_5m(&self) -> &str {
        self.load_5m.as_str()
    }

    pub fn get_load_15m(&self) -> &str {
        self.load_15m.as_str()
    }

    pub fn get_node_role(&self) -> &str {
        self.node_role.as_str()
    }

    pub fn get_master(&self) -> &str {
        self.master.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl TestExample<Cluster> for Cluster {
    fn test_example(_value: Option<&str>) -> Cluster {
        Cluster::builder()
            .ip("172.19.0.2".to_string())
            .heap_percent("32".to_string())
            .ram_percent("67".to_string())
            .cpu("2".to_string())
            .load_1m("0.00".to_string())
            .load_5m("0.05".to_string())
            .load_15m("0.05".to_string())
            .node_role("cdfhilmrstw".to_string())
            .master("*".to_string())
            .name("d93df49fa6ff".to_string())
            .build()
            .unwrap()
    }
}
