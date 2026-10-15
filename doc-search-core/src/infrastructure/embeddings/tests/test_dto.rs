use crate::infrastructure::embeddings::dto::{EmbeddingsRequest, EmbeddingsResponse};

#[test]
fn test_request_serialization() {
    let request = EmbeddingsRequest {
        text: "hello".to_string(),
        model: "bge".to_string(),
        overlap: 0.2,
    };
    let value = serde_json::to_value(&request).unwrap();
    assert_eq!("hello", value["text"]);
    assert_eq!("bge", value["model"]);
    assert_eq!(0.2, value["overlap"]);
}

#[test]
fn test_response_deserialization() {
    let response: EmbeddingsResponse =
        serde_json::from_str(r#"{ "embedding": [0.1, 0.2, 0.3] }"#).unwrap();
    assert_eq!(vec![0.1, 0.2, 0.3], response.embedding);
}
