use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct RedisConfig {
    #[getset(get, vis = "pub")]
    address: String,
    #[getset(get, vis = "pub")]
    username: String,
    #[getset(get, vis = "pub")]
    password: String,
    #[getset(get_copy, vis = "pub")]
    expired: u64,
}
