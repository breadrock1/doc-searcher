use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, CopyGetters, Getters)]
<<<<<<<< HEAD:src/infrastructure/vectorizer/config.rs
pub struct VectorizerConfig {
========
pub struct BAAIConfig {
>>>>>>>> master:src/tokenizer/baai/config.rs
    #[getset(get = "pub")]
    address: String,
    #[getset(get_copy = "pub")]
    is_truncate: bool,
    #[getset(get_copy = "pub")]
    is_normalize: bool,
}
