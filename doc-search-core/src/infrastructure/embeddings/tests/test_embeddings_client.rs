use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::ServiceConnect;
use crate::application::services::embeddings::ITokenizer;
use crate::infrastructure::embeddings::EmbeddingsClient;
use crate::infrastructure::embeddings::tests::common::{
    build_embeddings_config_for, json_response, start_test_server,
};

#[tokio::test]
async fn test_compute_success() -> anyhow::Result<()> {
    let (address, hits, _) =
        start_test_server(Arc::new(move |_| (200, json_response(vec![0.1, 0.2, 0.3])))).await;

    let config = build_embeddings_config_for(&address); // dimension 3, retries 2
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert_eq!(vec![0.1, 0.2, 0.3], *result);
    assert_eq!(1, hits.load(Ordering::SeqCst));
    Ok(())
}

#[tokio::test]
async fn test_compute_sends_openai_request() -> anyhow::Result<()> {
    let (address, _hits, head) =
        start_test_server(Arc::new(move |_| (200, json_response(vec![0.1, 0.2, 0.3])))).await;

    let config = build_embeddings_config_for(&address);
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert_eq!(vec![0.1, 0.2, 0.3], *result);

    let head = head.lock().unwrap();
    assert!(
        head.starts_with("POST /embeddings HTTP/1.1"),
        "unexpected request line: {head}"
    );
    assert!(
        head.contains("authorization: Bearer secret-key"),
        "missing authorization header: {head}"
    );
    assert!(
        head.contains("\"model\":\"bge\""),
        "missing model in request body: {head}"
    );
    assert!(
        head.contains("\"input\":\"hello\""),
        "missing input in request body: {head}"
    );
    Ok(())
}

#[tokio::test]
async fn test_compute_dimension_mismatch_degrades_to_empty() -> anyhow::Result<()> {
    let (address, hits, _) =
        start_test_server(Arc::new(move |_| (200, json_response(vec![0.1])))).await;

    let config = build_embeddings_config_for(&address);
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert!(result.is_empty());
    assert_eq!(1, hits.load(Ordering::SeqCst));
    Ok(())
}

#[tokio::test]
async fn test_compute_retries_on_rate_limit_then_success() -> anyhow::Result<()> {
    let (address, hits, _) = start_test_server(Arc::new(move |attempt| {
        if attempt <= 2 {
            (429, b"rate limited".to_vec())
        } else {
            (200, json_response(vec![1.0, 2.0, 3.0]))
        }
    }))
    .await;

    let config = build_embeddings_config_for(&address);
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert_eq!(vec![1.0, 2.0, 3.0], *result);
    assert!(hits.load(Ordering::SeqCst) >= 3);
    Ok(())
}

#[tokio::test]
async fn test_compute_server_error_degrades_to_empty_without_retry() -> anyhow::Result<()> {
    // Через openai_dive 5xx не ретраится: статус скрывается, ошибка становится
    // не-retriable (ParseError/InvalidRequestError) ещё до нашей retry-логики.
    let (address, hits, _) =
        start_test_server(Arc::new(move |_| (500, b"service unavailable".to_vec()))).await;

    let config = build_embeddings_config_for(&address);
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert!(result.is_empty());
    assert_eq!(1, hits.load(Ordering::SeqCst));
    Ok(())
}

#[tokio::test]
async fn test_compute_connection_refused_degrades_to_empty() -> anyhow::Result<()> {
    // Сетевой отказ ретраится, но после исчерпания retry клиент деградирует к пустому Tokens.
    let dead_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let dead_address = dead_listener.local_addr()?;
    drop(dead_listener);

    let config = build_embeddings_config_for(&format!("http://{dead_address}"));
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert!(result.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_compute_invalid_body_degrades_to_empty() -> anyhow::Result<()> {
    let (address, hits, _) = start_test_server(Arc::new(|_| {
        (
            200,
            serde_json::json!({ "embedding": 42 })
                .to_string()
                .into_bytes(),
        )
    }))
    .await;

    let config = build_embeddings_config_for(&address);
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert!(result.is_empty());
    assert_eq!(1, hits.load(Ordering::SeqCst));
    Ok(())
}
