use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Getset)]
pub struct OSearchConfig {
    #[getset(get, vis = "pub")]
    address: String,
    #[getset(get, vis = "pub")]
    username: String,
    #[getset(get, vis = "pub")]
    password: String,
    #[getset(get, vis = "pub")]
    semantic: OSearchKnnConfig,
    #[getset(get, vis = "pub")]
    cluster: OSearchClusterConfig,
}

#[derive(Clone, Debug, Deserialize, Getset)]
pub struct OSearchClusterConfig {
    #[getset(get_copy, vis = "pub")]
    number_of_shards: usize,
    #[getset(get_copy, vis = "pub")]
    number_of_replicas: usize,
}

#[derive(Clone, Debug, Deserialize, Getset)]
pub struct OSearchKnnConfig {
    #[getset(get, vis = "pub")]
    model_id: String,
    #[getset(get_copy, vis = "pub")]
    knn_dimension: u32,
    #[getset(get_copy, vis = "pub")]
    token_limit: u32,
    #[getset(get_copy, vis = "pub")]
    overlap_rate: f32,
    #[getset(get_copy, vis = "pub")]
    knn_ef_searcher: Option<u32>,
}
