use crate::infrastructure::embeddings::config::EmbeddingsConfig;
use crate::infrastructure::embeddings::tests::common::fixture::config::build_embeddings_config;

#[test]
fn test_embeddings_config_fields() {
    let config = build_embeddings_config();
    assert_eq!("http://localhost:8081", config.address());
    assert_eq!(1, config.timeout());
    assert_eq!(2, config.retries());
    assert_eq!("bge", config.model());
    assert_eq!(0.2, config.overlap());
    assert_eq!(3, config.dimension());
}

#[test]
fn test_embeddings_config_requires_all_fields() {
    let result: Result<EmbeddingsConfig, _> =
        serde_json::from_value(serde_json::json!({ "address": "http://x" }));
    assert!(result.is_err());
}
