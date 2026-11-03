use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Deserialize, Serialize)]
pub struct RerankerRequest {
    pub model: String,
    pub query: String,
    pub documents: Vec<String>,
    pub top_n: u32,
    pub truncate_prompt_tokens: i32,
}

#[derive(Builder, Deserialize, Serialize)]
pub struct RerankedDocuments {
    pub results: Vec<RerankedResult>,
}

#[derive(Clone, Builder, Deserialize, Serialize)]
pub struct RerankedResult {
    pub index: u32,
    pub relevance_score: f64,
}
