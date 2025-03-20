use getset::{CopyGetters, Getters};
use serde::Deserialize;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
pub struct ServerConfig {
    #[getset(get = "pub")]
    address: String,
}
