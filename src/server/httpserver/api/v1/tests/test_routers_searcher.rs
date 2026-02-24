use anyhow::anyhow;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum_test::http::header::CONTENT_TYPE;
use doc_search_core::domain::searcher::models::Pagination;
use doc_search_core::domain::searcher::SearchError;
use serde_json::Value;
use tower::ServiceExt;

use crate::server::httpserver::api::v1::API_VERSION_URL;
use crate::server::httpserver::tests::context::test_server;
use crate::server::httpserver::tests::mocks::searcher::MockSearcherService;
use crate::server::httpserver::tests::mocks::storage::MockStorageService;

use super::stubs;
use super::stubs::constants::SCROLL_ID;
use super::RESPONSE_BODY_SIZE_LIMIT;

#[tokio::test]
#[rstest::rstest]
#[case(
    stubs::fulltext_search_params_json_object(),
    StatusCode::OK,
    stubs::pagination_result_json_object()
)]
#[case(
    stubs::fulltext_search_params_with_filter_json_object(),
    StatusCode::OK,
    stubs::pagination_result_json_object()
)]
#[case(
    stubs::fulltext_search_params_json_object(),
    StatusCode::BAD_REQUEST,
    stubs::bad_request_error_json_response()
)]
#[case(
    stubs::fulltext_search_params_json_object(),
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_search_fulltext(
    #[case] request_body: Value,
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let storage = MockStorageService::new();
    let mut searcher = MockSearcherService::new();

    let expectation = searcher.expect_search().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |_| {
            let scroll = Some(SCROLL_ID.to_string());
            let documents = vec![
                stubs::founded_document_with_part_id(1),
                stubs::founded_document_with_part_id(2),
            ];

            Ok(Pagination::new(scroll, documents))
        }),
        StatusCode::BAD_REQUEST => expectation.returning(move |_| {
            let err = anyhow!("bad request");
            Err(SearchError::ValidationError(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_| {
            let err = anyhow!("internal server error");
            Err(SearchError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let request_body = serde_json::to_vec(&request_body).expect("failed to serialize json");
    let request = Request::builder()
        .method(Method::POST)
        .uri(format!("{}/search/fulltext", API_VERSION_URL))
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(request_body))
        .expect("failed to build request");

    let response = test_server_context
        .test_server
        .clone()
        .oneshot(request)
        .await?;
    assert_eq!(response.status(), expected_status);

    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("body should be ok");
    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}

#[tokio::test]
#[rstest::rstest]
#[case(
    stubs::semantic_search_params_json_object(),
    StatusCode::OK,
    stubs::pagination_result_json_object()
)]
#[case(
    stubs::semantic_search_params_with_tokens_json_object(),
    StatusCode::OK,
    stubs::pagination_result_json_object()
)]
#[case(
    stubs::semantic_search_params_with_filter_json_object(),
    StatusCode::OK,
    stubs::pagination_result_json_object()
)]
#[case(
    stubs::semantic_search_params_json_object(),
    StatusCode::BAD_REQUEST,
    stubs::bad_request_error_json_response()
)]
#[case(
    stubs::semantic_search_params_json_object(),
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_search_semantic(
    #[case] request_body: Value,
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let storage = MockStorageService::new();
    let mut searcher = MockSearcherService::new();

    let expectation = searcher.expect_search().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |_| {
            let scroll = Some(SCROLL_ID.to_string());
            let documents = vec![
                stubs::founded_document_with_part_id(1),
                stubs::founded_document_with_part_id(2),
            ];

            Ok(Pagination::new(scroll, documents))
        }),
        StatusCode::BAD_REQUEST => expectation.returning(move |_| {
            let err = anyhow!("bad request");
            Err(SearchError::ValidationError(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_| {
            let err = anyhow!("internal server error");
            Err(SearchError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let request_body = serde_json::to_vec(&request_body).expect("failed to serialize json");
    let request = Request::builder()
        .method(Method::POST)
        .uri(format!("{}/search/semantic", API_VERSION_URL))
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(request_body))
        .expect("failed to build request");

    let response = test_server_context
        .test_server
        .clone()
        .oneshot(request)
        .await?;
    assert_eq!(response.status(), expected_status);

    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("body should be ok");
    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}

#[tokio::test]
#[rstest::rstest]
#[case(
    stubs::hybrid_search_params_json_object(),
    StatusCode::OK,
    stubs::pagination_result_json_object()
)]
#[case(
    stubs::hybrid_search_params_with_filter_json_object(),
    StatusCode::OK,
    stubs::pagination_result_json_object()
)]
#[case(
    stubs::hybrid_search_params_json_object(),
    StatusCode::BAD_REQUEST,
    stubs::bad_request_error_json_response()
)]
#[case(
    stubs::hybrid_search_params_json_object(),
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_search_hybrid(
    #[case] request_body: Value,
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let storage = MockStorageService::new();
    let mut searcher = MockSearcherService::new();

    let expectation = searcher.expect_search().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |_| {
            let scroll = Some(SCROLL_ID.to_string());
            let documents = vec![
                stubs::founded_document_with_part_id(1),
                stubs::founded_document_with_part_id(2),
            ];

            Ok(Pagination::new(scroll, documents))
        }),
        StatusCode::BAD_REQUEST => expectation.returning(move |_| {
            let err = anyhow!("bad request");
            Err(SearchError::ValidationError(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_| {
            let err = anyhow!("internal server error");
            Err(SearchError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let request_body = serde_json::to_vec(&request_body).expect("failed to serialize json");
    let request = Request::builder()
        .method(Method::POST)
        .uri(format!("{}/search/hybrid", API_VERSION_URL))
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(request_body))
        .expect("failed to build request");

    let response = test_server_context
        .test_server
        .clone()
        .oneshot(request)
        .await?;
    assert_eq!(response.status(), expected_status);

    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("body should be ok");
    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}
