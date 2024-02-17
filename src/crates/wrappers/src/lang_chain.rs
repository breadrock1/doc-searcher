use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Builder, Default, Clone)]
pub struct LangChainTokens {
    #[serde(alias = "my-index")]
    my_index: String,
    content: String,
    content_vector: Vec<String>,
}
