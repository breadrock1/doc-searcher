use crate::infrastructure::vllm::config::VLLMConfig;

pub fn build_embeddings_config() -> VLLMConfig {
    serde_json::from_value(serde_json::json!({
        "base_url": "http://localhost:8081/v1",
        "api_key": "secret-key",
        "timeout": 1,
        "retries": 2,
        "model": "bge",
        "overlap": 0.2,
        "dimension": 3,
    }))
    .expect("failed to deserialize vllm config")
}

pub fn build_embeddings_config_for(base_url: &str) -> VLLMConfig {
    serde_json::from_value(serde_json::json!({
        "base_url": base_url,
        "api_key": "secret-key",
        "timeout": 1,
        "retries": 2,
        "model": "bge",
        "overlap": 0.2,
        "dimension": 3,
    }))
    .expect("failed to deserialize vllm config")
}
