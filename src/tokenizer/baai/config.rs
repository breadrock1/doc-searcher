use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
pub struct BAAIConfig {
    #[getset(get = "pub")]
    address: String,
    #[getset(get_copy = "pub")]
    is_truncate: bool,
    #[getset(get_copy = "pub")]
    is_normalize: bool,
}
