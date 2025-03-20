use getset::{CopyGetters, Getters};
use serde::Deserialize;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
#[getset(get = "pub")]
pub struct ServerConfig {
    address: String,
}
