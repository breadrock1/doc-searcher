use getset::Getters;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct OSearchConfig {
    address: String,
    username: String,
    password: String,
}
