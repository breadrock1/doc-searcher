use crate::shared::kernel::Tokens;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct EmbeddingsRequest {
    pub text: String,
    pub model: String,
    pub overlap: f64,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingsResponse {
    pub embedding: Vec<f64>,
}

impl From<EmbeddingsResponse> for Tokens {
    fn from(resp: EmbeddingsResponse) -> Self {
        Tokens::from(resp.embedding)
    }
}
