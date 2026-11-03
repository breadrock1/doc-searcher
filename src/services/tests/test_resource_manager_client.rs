use std::sync::Arc;

use serde_json::json;

use doc_search_core::shared::kernel::ResourceId;
use doc_search_core::shared::services::{IResourceManager, ResourceManagerError};
use doc_search_core::ServiceConnect;

use crate::services::resource_manager::{ResourceManagerClient, ResourceManagerConfig};
use crate::services::tests::common::start_test_server;

const RESOURCE_ID: &str = "53079efa-b0f2-4058-96dc-c792531972db";
const RESOURCE_PREFIX: &str = "53079efa-b0f2";
const INDEX_ID: &str = "test-folder";

fn config_for(address: &str) -> ResourceManagerConfig {
    serde_json::from_value(json!({
        "base_url": address,
        "timeout": 5,
    }))
    .expect("failed to build resource manager config")
}

async fn build_client(address: &str) -> ResourceManagerClient {
    ResourceManagerClient::connect(&config_for(address))
        .await
        .expect("failed to connect resource manager client")
}

fn resource_response() -> Vec<u8> {
    serde_json::to_vec(&json!({
        "id": RESOURCE_ID,
        "name": "Hully InCorp Resource",
        "created_at": "2025-10-01T10:30:00Z",
        "is_public": false,
        "external_id": INDEX_ID,
        "resource_type": "Document",
        "resource_owner": "User",
        "prefix": RESOURCE_PREFIX,
    }))
    .expect("failed to serialize resource response")
}

#[tokio::test]
async fn test_get_index_success() -> anyhow::Result<()> {
    let (address, hits, head) =
        start_test_server(Arc::new(move |_| (200, resource_response()))).await;

    let client = build_client(&address).await;
    let info = client
        .get_index(&ResourceId::from(RESOURCE_ID.to_string()))
        .await?;

    assert_eq!(INDEX_ID, info.index);
    assert_eq!(RESOURCE_PREFIX, info.prefix);
    assert_eq!(1, hits.load(std::sync::atomic::Ordering::SeqCst));

    let head = head.lock().expect("head lock");
    assert!(
        head.starts_with(&format!("GET /api/v1/resource/{RESOURCE_ID} HTTP/1.1")),
        "unexpected request line: {head}"
    );

    Ok(())
}

#[tokio::test]
async fn test_get_index_not_found() -> anyhow::Result<()> {
    let (address, _, _) =
        start_test_server(Arc::new(|_| (404, b"{\"message\":\"not found\"}".to_vec()))).await;

    let client = build_client(&address).await;
    let result = client
        .get_index(&ResourceId::from(RESOURCE_ID.to_string()))
        .await;

    assert!(matches!(result, Err(ResourceManagerError::NotFound(_))));

    Ok(())
}

#[tokio::test]
async fn test_get_index_server_error_is_internal() -> anyhow::Result<()> {
    let (address, _, _) = start_test_server(Arc::new(|_| (500, b"internal".to_vec()))).await;

    let client = build_client(&address).await;
    let result = client
        .get_index(&ResourceId::from(RESOURCE_ID.to_string()))
        .await;

    assert!(matches!(
        result,
        Err(ResourceManagerError::InternalError(_))
    ));

    Ok(())
}

#[tokio::test]
async fn test_get_index_without_external_id_is_internal_error() -> anyhow::Result<()> {
    let body = serde_json::to_vec(&json!({
        "external_id": null,
        "prefix": RESOURCE_PREFIX,
    }))
    .expect("failed to serialize resource response");
    let (address, _, _) = start_test_server(Arc::new(move |_| (200, body.clone()))).await;

    let client = build_client(&address).await;
    let result = client
        .get_index(&ResourceId::from(RESOURCE_ID.to_string()))
        .await;

    assert!(matches!(
        result,
        Err(ResourceManagerError::InternalError(_))
    ));

    Ok(())
}

#[tokio::test]
async fn test_get_index_invalid_json_is_internal_error() -> anyhow::Result<()> {
    let (address, _, _) = start_test_server(Arc::new(|_| (200, b"not-a-json".to_vec()))).await;

    let client = build_client(&address).await;
    let result = client
        .get_index(&ResourceId::from(RESOURCE_ID.to_string()))
        .await;

    assert!(matches!(
        result,
        Err(ResourceManagerError::InternalError(_))
    ));

    Ok(())
}
