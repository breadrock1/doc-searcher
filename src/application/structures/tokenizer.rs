use derive_builder::Builder;
use gset::Getset;

#[derive(Builder, Debug, Getset)]
pub struct InputContent {
    #[getset(get, vis = "pub")]
    content: String,
}

#[derive(Builder, Clone, Debug, Getset)]
pub struct TokenizedContent {
    #[getset(get, vis = "pub")]
    text: String,
    #[getset(get, vis = "pub")]
    embeddings: Vec<f64>,
}
