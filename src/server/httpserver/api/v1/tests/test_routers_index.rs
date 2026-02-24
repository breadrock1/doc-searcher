use anyhow::anyhow;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum_test::http::header::CONTENT_TYPE;
use serde_json::Value;
use tower::ServiceExt;

use doc_search_core::domain::storage::StorageError;

use crate::server::httpserver::api::v1::form::*;
use crate::server::httpserver::api::v1::router::index::STORAGE_ALL_INDEXES_URL;
use crate::server::httpserver::api::v1::tests::fixtures::form::*;
use crate::server::httpserver::api::v1::API_VERSION_URL;
use crate::server::httpserver::tests::context::test_server;
use crate::server::httpserver::tests::mocks::searcher::MockSearcherService;
use crate::server::httpserver::tests::mocks::storage::MockStorageService;

use super::stubs;
use super::{RESPONSE_BODY_SIZE_LIMIT, TEST_CONTENT_TYPE};

#[tokio::test]
#[rstest::rstest]
#[case(StatusCode::OK, stubs::get_all_indexes_json_object())]
#[case(StatusCode::BAD_REQUEST, stubs::bad_request_error_json_response())]
#[case(
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_get_all_indexes(
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let searcher = MockSearcherService::new();
    let mut storage = MockStorageService::new();

    let mocked = storage.expect_get_all_indexes().once();

    match expected_status {
        StatusCode::OK => mocked.returning(move || Ok(vec![TEST_INDEX_ID.to_string()])),
        StatusCode::BAD_REQUEST => mocked.returning(move || {
            let err = anyhow!("bad request");
            Err(StorageError::ValidationError(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => mocked.returning(move || {
            let err = anyhow!("internal server error");
            Err(StorageError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("{}{}", API_VERSION_URL, STORAGE_ALL_INDEXES_URL))
        .header(CONTENT_TYPE, TEST_CONTENT_TYPE)
        .body(Body::empty())
        .expect("failed to build request");

    let response = test_server_context
        .test_server
        .clone()
        .oneshot(request)
        .await?;
    assert_eq!(response.status(), expected_status);

    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("extracting response body failed");

    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}

#[tokio::test]
#[rstest::rstest]
#[case(StatusCode::OK, stubs::created_index_json_object())]
#[case(StatusCode::NOT_FOUND, stubs::not_found_error_json_response())]
#[case(StatusCode::BAD_REQUEST, stubs::bad_request_error_json_response())]
#[case(
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_get_index(
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let searcher = MockSearcherService::new();
    let mut storage = MockStorageService::new();

    let expectation = storage.expect_get_index().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |index_id| Ok(index_id.to_string())),
        StatusCode::BAD_REQUEST => expectation.returning(move |_| {
            let err = anyhow!("bad request");
            Err(StorageError::ValidationError(err))
        }),
        StatusCode::NOT_FOUND => expectation.returning(move |_| {
            let err = anyhow!("not found");
            Err(StorageError::IndexNotFound(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_| {
            let err = anyhow!("internal server error");
            Err(StorageError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("{}/storage/{}", API_VERSION_URL, TEST_INDEX_ID))
        .header(CONTENT_TYPE, TEST_CONTENT_TYPE)
        .body(Body::empty())
        .expect("failed to build request");

    let response = test_server_context
        .test_server
        .clone()
        .oneshot(request)
        .await?;
    assert_eq!(response.status(), expected_status);

    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("extracting response body failed");

    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}

#[tokio::test]
#[rstest::rstest]
#[case(
    create_index_form(),
    StatusCode::CREATED,
    stubs::created_index_json_object()
)]
#[case(
    create_index_form_with_knn(),
    StatusCode::CREATED,
    stubs::created_index_json_object()
)]
#[case(
    create_index_form_with_knn(),
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_create_index_route(
    #[case] request_form: CreateIndexForm,
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let searcher = MockSearcherService::new();
    let mut storage = MockStorageService::new();

    let expectation = storage.expect_create_index().once();

    match expected_status {
        StatusCode::CREATED => expectation.returning(move |params| Ok(params.id.clone())),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_| {
            let err = anyhow!("internal server error");
            Err(StorageError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let request_body = serde_json::to_vec(&request_form)?;
    let request = Request::builder()
        .method(Method::PUT)
        .uri(format!("{}/storage/{}", API_VERSION_URL, TEST_INDEX_ID))
        .header(CONTENT_TYPE, TEST_CONTENT_TYPE)
        .body(Body::from(request_body))
        .expect("couldn't build request");

    let response = test_server_context
        .test_server
        .clone()
        .oneshot(request)
        .await?;
    assert_eq!(response.status(), expected_status);

    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("extracting response body failed");

    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}

#[tokio::test]
#[rstest::rstest]
#[case(StatusCode::OK, stubs::success_json_response())]
#[case(StatusCode::NOT_FOUND, stubs::not_found_error_json_response())]
#[case(
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_delete_index(
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let searcher = MockSearcherService::new();
    let mut storage = MockStorageService::new();

    let expectation = storage.expect_delete_index().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |_| Ok(())),
        StatusCode::NOT_FOUND => expectation.returning(move |_| {
            let err = anyhow!("not found");
            Err(StorageError::IndexNotFound(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_| {
            let err = anyhow!("internal server error");
            Err(StorageError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let request = Request::builder()
        .method(Method::DELETE)
        .uri(format!("{}/storage/{}", API_VERSION_URL, TEST_INDEX_ID))
        .header(CONTENT_TYPE, TEST_CONTENT_TYPE)
        .body(Body::empty())
        .expect("failed to build request");

    let response = test_server_context
        .test_server
        .clone()
        .oneshot(request)
        .await?;
    assert_eq!(response.status(), expected_status);

    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT)
        .await
        .expect("extracting response body failed");

    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}
