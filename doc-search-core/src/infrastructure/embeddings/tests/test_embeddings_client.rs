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
    let (address, hits) =
        start_test_server(Arc::new(move |_| (200, json_response(vec![0.1, 0.2, 0.3])))).await;

    let config = build_embeddings_config_for(&address); // dimension 3, retries 2
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert_eq!(vec![0.1, 0.2, 0.3], *result);
    assert_eq!(1, hits.load(Ordering::SeqCst));
    Ok(())
}

#[tokio::test]
async fn test_compute_dimension_mismatch_degrades_to_empty() -> anyhow::Result<()> {
    let (address, _hits) =
        start_test_server(Arc::new(move |_| (200, json_response(vec![0.1])))).await;

    let config = build_embeddings_config_for(&address);
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert!(result.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_compute_retries_then_success() -> anyhow::Result<()> {
    let (address, hits) = start_test_server(Arc::new(move |attempt| {
        if attempt <= 2 {
            (500, b"boom".to_vec())
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
async fn test_compute_service_unavailable_degrades_to_empty() -> anyhow::Result<()> {
    let (address, hits) =
        start_test_server(Arc::new(move |_| (500, b"service unavailable".to_vec()))).await;

    let config = build_embeddings_config_for(&address);
    let client = EmbeddingsClient::connect(&config).await?;
    let result = client.compute("hello").await?;

    assert!(result.is_empty());
    assert_eq!(3, hits.load(Ordering::SeqCst));
    Ok(())
}
