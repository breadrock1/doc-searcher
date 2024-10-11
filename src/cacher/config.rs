use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
pub struct CacherConfig {
    #[getset(get = "pub")]
    address: String,
    #[getset(get = "pub")]
    username: String,
    #[getset(get = "pub")]
    password: String,
    #[getset(get_copy = "pub")]
    expired: u64,
}
