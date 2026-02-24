use anyhow::anyhow;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum_test::http::header::CONTENT_TYPE;
use serde_json::Value;
use tower::ServiceExt;

use doc_search_core::domain::searcher::models::Pagination;
use doc_search_core::domain::searcher::SearchError;
use doc_search_core::domain::storage::StorageError;

use crate::server::httpserver::api::v1::API_VERSION_URL;
use crate::server::httpserver::tests::context::test_server;
use crate::server::httpserver::tests::mocks::searcher::MockSearcherService;
use crate::server::httpserver::tests::mocks::storage::MockStorageService;

use super::{
    stubs,
    stubs::constants::{COMPOSITE_INDEX_IDS, LARGE_DOCUMENT_ID, TEST_INDEX_ID},
    RESPONSE_BODY_SIZE_LIMIT, TEST_CONTENT_TYPE,
};

#[tokio::test]
#[rstest::rstest]
#[case(StatusCode::OK, stubs::document_parts_json_object())]
#[case(StatusCode::NOT_FOUND, stubs::not_found_error_json_response())]
#[case(
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_get_document_parts(
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let searcher = MockSearcherService::new();
    let mut storage = MockStorageService::new();

    let expectation = storage.expect_get_document_parts().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |_, _| {
            Ok(vec![
                stubs::build_document_part(1),
                stubs::build_document_part(2),
            ])
        }),
        StatusCode::NOT_FOUND => expectation.returning(move |_, _| {
            let err = anyhow!("not found");
            Err(StorageError::IndexNotFound(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_, _| {
            let err = anyhow!("internal server error");
            Err(StorageError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let target_uri = format!(
        "{}/storage/{}/{}",
        API_VERSION_URL, TEST_INDEX_ID, LARGE_DOCUMENT_ID
    );
    let request = Request::builder()
        .method(Method::GET)
        .uri(target_uri)
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
    StatusCode::OK,
    TEST_INDEX_ID,
    stubs::retrieve_index_documents_params_json_object(),
    stubs::document_parts_json_object()
)]
#[case(
    StatusCode::OK,
    COMPOSITE_INDEX_IDS,
    stubs::retrieve_index_documents_params_json_object(),
    stubs::document_parts_json_object()
)]
#[case(
    StatusCode::OK,
    TEST_INDEX_ID,
    stubs::retrieve_index_documents_params_with_filter_json_object(),
    stubs::document_parts_json_object()
)]
#[case(
    StatusCode::NOT_FOUND,
    TEST_INDEX_ID,
    stubs::retrieve_index_documents_params_json_object(),
    stubs::not_found_error_json_response()
)]
#[case(
    StatusCode::INTERNAL_SERVER_ERROR,
    TEST_INDEX_ID,
    stubs::retrieve_index_documents_params_json_object(),
    stubs::internal_server_error_json_response()
)]
async fn test_get_index_documents(
    #[case] expected_status: StatusCode,
    #[case] indexes: &str,
    #[case] request_body: Value,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let storage = MockStorageService::new();
    let mut searcher = MockSearcherService::new();

    let expectation = searcher.expect_search().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |_| {
            let documents = vec![
                stubs::founded_document_with_part_id(1),
                stubs::founded_document_with_part_id(2),
            ];

            Ok(Pagination::new(None, documents))
        }),
        StatusCode::NOT_FOUND => expectation.returning(move |_| {
            let err = anyhow!("not found");
            Err(SearchError::IndexNotFound(err))
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
        .uri(format!("{}/storage/{}/documents", API_VERSION_URL, indexes))
        .header(CONTENT_TYPE, TEST_CONTENT_TYPE)
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
        .expect("extracting response body failed");

    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}

#[tokio::test]
#[rstest::rstest]
#[case(
    StatusCode::CREATED,
    stubs::create_document_json_object(),
    stubs::stored_document_info_json_object()
)]
#[case(
    StatusCode::BAD_REQUEST,
    stubs::create_document_json_object(),
    stubs::bad_request_error_json_response()
)]
#[case(
    StatusCode::NOT_FOUND,
    stubs::create_document_json_object(),
    stubs::not_found_error_json_response()
)]
#[case(
    StatusCode::CONFLICT,
    stubs::create_document_json_object(),
    stubs::conflict_error_json_response()
)]
#[case(
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::create_document_json_object(),
    stubs::internal_server_error_json_response()
)]
async fn test_store_document(
    #[case] expected_status: StatusCode,
    #[case] request_body: Value,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let searcher = MockSearcherService::new();
    let mut storage = MockStorageService::new();

    storage
        .expect_get_index()
        .returning(|_| Ok(TEST_INDEX_ID.to_string()));

    let expectation = storage.expect_store_document_parts().once();

    match expected_status {
        StatusCode::CREATED => expectation.returning(move |_, _| Ok(stubs::stored_document_info())),
        StatusCode::BAD_REQUEST => expectation.returning(move |_, _| {
            let err = anyhow!("bad request");
            Err(StorageError::ValidationError(err))
        }),
        StatusCode::NOT_FOUND => expectation.returning(move |_, _| {
            let err = anyhow!("not found");
            Err(StorageError::IndexNotFound(err))
        }),
        StatusCode::CONFLICT => expectation.returning(move |_, _| {
            let err = anyhow!("conflict error");
            Err(StorageError::DocumentAlreadyExists(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_, _| {
            let err = anyhow!("internal server error");
            Err(StorageError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let target_uri = format!("{}/storage/{}/create", API_VERSION_URL, TEST_INDEX_ID);
    let request_body = serde_json::to_vec(&request_body).expect("failed to serialize json");
    let request = Request::builder()
        .method(Method::PUT)
        .uri(target_uri)
        .header(CONTENT_TYPE, TEST_CONTENT_TYPE)
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
        .expect("extracting response body failed");

    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}

#[tokio::test]
#[rstest::rstest]
#[case(
    StatusCode::CREATED,
    stubs::create_documents_json_object(),
    stubs::stored_documents_info_json_object()
)]
#[case(
    StatusCode::BAD_REQUEST,
    stubs::create_documents_json_object(),
    stubs::bad_request_error_json_response()
)]
#[case(
    StatusCode::NOT_FOUND,
    stubs::create_documents_json_object(),
    stubs::not_found_error_json_response()
)]
#[case(
    StatusCode::CONFLICT,
    stubs::create_documents_json_object(),
    stubs::conflict_error_json_response()
)]
#[case(
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::create_documents_json_object(),
    stubs::internal_server_error_json_response()
)]
async fn test_store_documents(
    #[case] expected_status: StatusCode,
    #[case] request_body: Value,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let searcher = MockSearcherService::new();
    let mut storage = MockStorageService::new();

    storage
        .expect_get_index()
        .returning(|index_id| Ok(index_id.to_string()));

    let expectation = storage.expect_store_document_parts().times(1);

    match expected_status {
        StatusCode::CREATED => expectation.returning(move |_, _| Ok(stubs::stored_document_info())),
        StatusCode::BAD_REQUEST => expectation.returning(move |_, _| {
            let err = anyhow!("bad request");
            Err(StorageError::ValidationError(err))
        }),
        StatusCode::NOT_FOUND => expectation.returning(move |_, _| {
            let err = anyhow!("not found");
            Err(StorageError::IndexNotFound(err))
        }),
        StatusCode::CONFLICT => expectation.returning(move |_, _| {
            let err = anyhow!("conflict error");
            Err(StorageError::DocumentAlreadyExists(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_, _| {
            let err = anyhow!("internal server error");
            Err(StorageError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let target_uri = format!("{}/storage/{}/documents", API_VERSION_URL, TEST_INDEX_ID);
    let request_body = serde_json::to_vec(&request_body).expect("failed to serialize json");
    let request = Request::builder()
        .method(Method::PUT)
        .uri(target_uri)
        .header(CONTENT_TYPE, TEST_CONTENT_TYPE)
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
async fn test_delete_document_parts(
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let searcher = MockSearcherService::new();
    let mut storage = MockStorageService::new();

    let expectation = storage.expect_delete_document_parts().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |_, _| Ok(())),
        StatusCode::NOT_FOUND => expectation.returning(move |_, _| {
            let err = anyhow!("not found");
            Err(StorageError::IndexNotFound(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_, _| {
            let err = anyhow!("internal server error");
            Err(StorageError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let target_uri = format!(
        "{}/storage/{}/{}",
        API_VERSION_URL, TEST_INDEX_ID, LARGE_DOCUMENT_ID
    );
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(target_uri)
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
