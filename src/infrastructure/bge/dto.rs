use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::application::structures::{InputContent, TokenizedContent, TokenizedContentBuilder};

#[derive(Builder, Debug, Serialize)]
pub struct InputForm {
    content: String,
    model: String,
}

impl From<&InputContent> for InputForm {
    fn from(input: &InputContent) -> Self {
        InputFormBuilder::default()
            .content(input.content().clone())
            .model("bge".to_string())
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Debug, Deserialize)]
pub struct ResultForm {
    text: String,
    data: Vec<TokensData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TokensData {
    embedding: Vec<Vec<f64>>,
}

impl ResultForm {
    pub fn get_embeddings(&self) -> Vec<f64> {
        self.data
            .first()
            .unwrap()
            .clone()
            .embedding
            .first()
            .unwrap()
            .clone()
    }
}

impl From<ResultForm> for TokenizedContent {
    fn from(form: ResultForm) -> Self {
        let embeddings = form.get_embeddings();
        TokenizedContentBuilder::default()
            .text(form.text)
            .embeddings(embeddings)
            .build()
            .unwrap()
    }
}
