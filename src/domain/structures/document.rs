use getset::{CopyGetters, Getters};

#[derive(Getters, CopyGetters)]
pub struct Document {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    file_name: String,
    #[getset(get = "pub")]
    file_path: String,
    #[getset(get_copy = "pub")]
    file_size: u32,
    #[getset(get = "pub")]
    content: String,
    #[getset(get = "pub")]
    created_at: i64,
    #[getset(get = "pub")]
    modified_at: i64,
    #[getset(get = "pub")]
    embeddings: Vec<EmbeddingChunk>,
}

#[derive(Getters, CopyGetters)]
pub struct EmbeddingChunk {
    #[getset(get_copy = "pub")]
    chunk_id: u32,
    #[getset(get = "pub")]
    chunk_text: String,
    #[getset(get = "pub")]
    tokens: Vec<f64>,
}
