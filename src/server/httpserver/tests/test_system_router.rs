use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum_test::http::header::CONTENT_TYPE;
use tower::ServiceExt;

use crate::server::httpserver::tests::context::test_server::create_test_server_context;
use crate::server::httpserver::tests::mocks::searcher::MockSearcherService;
use crate::server::httpserver::tests::mocks::storage::MockStorageService;

const HOME_URL: &str = "/";
const METRICS_URL: &str = "/api/metrics";
const INDEX_HTML_PAGE_DATA: &str = include_str!("../../../../static/index.html");
const RESPONSE_BODY_SIZE_LIMIT: usize = 10 * 1024 * 1024;

#[tokio::test]
async fn test_home_returns_index_page() -> anyhow::Result<()> {
    let test_context =
        create_test_server_context(MockStorageService::new(), MockSearcherService::new());

    let request = Request::builder()
        .method(Method::GET)
        .uri(HOME_URL)
        .body(Body::empty())
        .expect("failed to build home request");

    let response = test_context.test_server.clone().oneshot(request).await?;
    assert_eq!(StatusCode::OK, response.status());

    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("extracting home response body failed");
    assert_eq!(INDEX_HTML_PAGE_DATA.as_bytes(), &body[..]);

    Ok(())
}

#[tokio::test]
async fn test_metrics_returns_prometheus_text() -> anyhow::Result<()> {
    let test_context =
        create_test_server_context(MockStorageService::new(), MockSearcherService::new());

    let request = Request::builder()
        .method(Method::GET)
        .uri(METRICS_URL)
        .body(Body::empty())
        .expect("failed to build metrics request");

    let response = test_context.test_server.clone().oneshot(request).await?;
    assert_eq!(StatusCode::OK, response.status());

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .expect("metrics response must have content-type");
    assert_eq!("text/plain", content_type);

    let _body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("extracting metrics response body failed");

    Ok(())
}
