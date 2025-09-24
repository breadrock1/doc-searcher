use gset::Getset;
use serde::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct BgeConfig {
    #[getset(get, vis = "pub")]
    address: String,
    // TODO: need to remove this configs
    #[getset(get_copy, vis = "pub")]
    is_truncate: bool,
    #[getset(get_copy, vis = "pub")]
    is_normalize: bool,
}
