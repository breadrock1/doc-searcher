use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

#[derive(Deserialize, CopyGetters, Getters)]
#[getset(get = "pub")]
pub struct RedisConfig {
    address: String,
    username: String,
    password: String,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    expired: u64,
}
