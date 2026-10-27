use crate::infrastructure::embeddings::config::EmbeddingsConfig;

pub fn build_embeddings_config() -> EmbeddingsConfig {
    serde_json::from_value(serde_json::json!({
        "base_url": "http://localhost:8081/v1",
        "api_key": "secret-key",
        "timeout": 1,
        "retries": 2,
        "model": "bge",
        "overlap": 0.2,
        "dimension": 3,
    }))
    .expect("failed to deserialize embeddings config")
}

pub fn build_embeddings_config_for(base_url: &str) -> EmbeddingsConfig {
    serde_json::from_value(serde_json::json!({
        "base_url": base_url,
        "api_key": "secret-key",
        "timeout": 1,
        "retries": 2,
        "model": "bge",
        "overlap": 0.2,
        "dimension": 3,
    }))
    .expect("failed to deserialize embeddings config")
}
