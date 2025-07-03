pub struct Document {
    pub id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: u32,
    pub content: String,
    pub created_at: i64,
    pub modified_at: i64,
    pub embeddings: Vec<EmbeddingChunk>,
}

pub struct EmbeddingChunk {
    pub chunk_id: u32,
    pub chunk_text: String,
    pub tokens: Vec<f64>,
}
