use derive_builder::Builder;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Getters, Deserialize, Serialize)]
pub struct Tokens {
    #[getset(get = "pub")]
    tokens: Vec<f64>,
}

impl Tokens {
    pub fn builder() -> TokensBuilder {
        TokensBuilder::default()
    }
}
