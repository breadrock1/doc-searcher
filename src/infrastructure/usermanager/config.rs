use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct UserManagerConfig {
    #[getset(get, vis = "pub")]
    address: String,
}
