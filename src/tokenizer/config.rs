use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

use crate::tokenizer::baai::config::BAAIConfig;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
#[getset(get = "pub")]
pub struct TokenizerConfig {
    baai: BAAIConfig,
}
