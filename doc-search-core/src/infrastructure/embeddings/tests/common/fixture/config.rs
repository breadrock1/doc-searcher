use crate::infrastructure::embeddings::config::EmbeddingsConfig;

pub fn build_embeddings_config() -> EmbeddingsConfig {
    serde_json::from_value(serde_json::json!({
        "address": "http://localhost:8081",
        "timeout": 1,
        "retries": 2,
        "model": "bge",
        "overlap": 0.2,
        "dimension": 3,
    }))
    .expect("failed to deserialize embeddings config")
}

pub fn build_embeddings_config_for(address: &str) -> EmbeddingsConfig {
    serde_json::from_value(serde_json::json!({
        "address": address,
        "timeout": 1,
        "retries": 2,
        "model": "bge",
        "overlap": 0.2,
        "dimension": 3,
    }))
    .expect("failed to deserialize embeddings config")
}
