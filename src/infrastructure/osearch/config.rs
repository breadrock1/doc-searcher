use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct OSearchConfig {
    address: String,
    username: String,
    password: String,
    semantic: OSearchKnnConfig,
    cluster: OSearchClusterConfig,
}

#[derive(Clone, Debug, Deserialize, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct OSearchClusterConfig {
    number_of_shards: usize,
    number_of_replicas: usize,
}

#[derive(Clone, Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct OSearchKnnConfig {
    model_id: String,
}
