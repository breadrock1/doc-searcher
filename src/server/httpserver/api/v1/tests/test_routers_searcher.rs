use anyhow::anyhow;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum_test::http::header::CONTENT_TYPE;
use doc_search_core::domain::searcher::models::Pagination;
use doc_search_core::domain::searcher::SearchError;
use doc_search_core::shared::services::ResourceManagerError;
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

use crate::server::httpserver::api::v1::API_VERSION_URL;
use crate::server::httpserver::tests::context::test_server;
use crate::server::httpserver::tests::mocks::resource_manager::{
    MockResourceManagerService, StubResourceManager,
};
use crate::server::httpserver::tests::mocks::searcher::MockSearcherService;
use crate::server::httpserver::tests::mocks::storage::MockStorageService;

use super::stubs;
use super::stubs::constants::{SCROLL_ID, TEST_INDEX_ID};
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
        StatusCode::OK => expectation.returning(move |_, _| {
            let scroll = Some(SCROLL_ID.to_string());
            let documents = vec![
                stubs::founded_document_with_part_id(1),
                stubs::founded_document_with_part_id(2),
            ];

            Ok(Pagination::new(scroll, documents))
        }),
        StatusCode::BAD_REQUEST => expectation.returning(move |_, _| {
            let err = anyhow!("bad request");
            Err(SearchError::ValidationError(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_, _| {
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
        StatusCode::OK => expectation.returning(move |_, _| {
            let scroll = Some(SCROLL_ID.to_string());
            let documents = vec![
                stubs::founded_document_with_part_id(1),
                stubs::founded_document_with_part_id(2),
            ];

            Ok(Pagination::new(scroll, documents))
        }),
        StatusCode::BAD_REQUEST => expectation.returning(move |_, _| {
            let err = anyhow!("bad request");
            Err(SearchError::ValidationError(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_, _| {
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
        StatusCode::OK => expectation.returning(move |_, _| {
            let scroll = Some(SCROLL_ID.to_string());
            let documents = vec![
                stubs::founded_document_with_part_id(1),
                stubs::founded_document_with_part_id(2),
            ];

            Ok(Pagination::new(scroll, documents))
        }),
        StatusCode::BAD_REQUEST => expectation.returning(move |_, _| {
            let err = anyhow!("bad request");
            Err(SearchError::ValidationError(err))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_, _| {
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

#[tokio::test]
#[rstest::rstest]
#[case(StatusCode::OK, stubs::pagination_result_json_object())]
#[case(
    StatusCode::INTERNAL_SERVER_ERROR,
    stubs::internal_server_error_json_response()
)]
async fn test_paginate_next(
    #[case] expected_status: StatusCode,
    #[case] expected_body: Value,
) -> anyhow::Result<()> {
    let storage = MockStorageService::new();
    let mut searcher = MockSearcherService::new();

    let expectation = searcher.expect_paginate().once();

    match expected_status {
        StatusCode::OK => expectation.returning(move |_| {
            let scroll = Some(SCROLL_ID.to_string());
            let documents = vec![
                stubs::founded_document_with_part_id(1),
                stubs::founded_document_with_part_id(2),
            ];

            Ok(Pagination::new(scroll, documents))
        }),
        StatusCode::INTERNAL_SERVER_ERROR => expectation.returning(move |_| {
            let err = anyhow!("internal server error");
            Err(SearchError::InternalError(err))
        }),
        _ => return Err(anyhow!("unexpected test case")),
    };

    let test_server_context = test_server::create_test_server_context(storage, searcher);

    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("{}/search/paginate/{}", API_VERSION_URL, SCROLL_ID))
        .header(CONTENT_TYPE, "application/json")
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
        .expect("body should be ok");
    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");
    assert_eq!(expected_body, data);

    Ok(())
}

async fn send_fulltext_search(
    context: &test_server::TestServerContext,
    body: Value,
) -> anyhow::Result<(StatusCode, Value)> {
    let request_body = serde_json::to_vec(&body).expect("failed to serialize json");
    let request = Request::builder()
        .method(Method::POST)
        .uri(format!("{}/search/fulltext", API_VERSION_URL))
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(request_body))
        .expect("failed to build request");

    let response = context.test_server.clone().oneshot(request).await?;
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), RESPONSE_BODY_SIZE_LIMIT).await?;
    let data = serde_json::from_slice::<Value>(&body).expect("failed to parse json");

    Ok((status, data))
}

#[tokio::test]
async fn test_search_resource_not_found_returns_404() -> anyhow::Result<()> {
    let mut resource_manager = MockResourceManagerService::new();
    resource_manager.expect_get_index().once().returning(|_| {
        Err(ResourceManagerError::NotFound(
            "resource not found".to_string(),
        ))
    });

    let context = test_server::create_test_server_context_with_resource_manager(
        MockStorageService::new(),
        MockSearcherService::new(),
        Arc::new(resource_manager),
    );

    let (status, body) =
        send_fulltext_search(&context, stubs::fulltext_search_params_json_object()).await?;

    assert_eq!(StatusCode::NOT_FOUND, status);
    assert_eq!(json!(404), body["status"]);
    assert_eq!(json!("resource not found"), body["message"]);

    Ok(())
}

#[tokio::test]
async fn test_search_resource_manager_error_returns_500() -> anyhow::Result<()> {
    let mut resource_manager = MockResourceManagerService::new();
    resource_manager
        .expect_get_index()
        .once()
        .returning(|_| Err(ResourceManagerError::InternalError("boom".to_string())));

    let context = test_server::create_test_server_context_with_resource_manager(
        MockStorageService::new(),
        MockSearcherService::new(),
        Arc::new(resource_manager),
    );

    let (status, body) =
        send_fulltext_search(&context, stubs::fulltext_search_params_json_object()).await?;

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, status);
    assert_eq!(json!(500), body["status"]);
    assert_eq!(json!("boom"), body["message"]);

    Ok(())
}

#[tokio::test]
async fn test_search_rejects_empty_resource_id() -> anyhow::Result<()> {
    let context = test_server::create_test_server_context(
        MockStorageService::new(),
        MockSearcherService::new(),
    );

    let mut body = stubs::fulltext_search_params_json_object();
    body["resource_id"] = json!("");

    let (status, body) = send_fulltext_search(&context, body).await?;

    assert_eq!(StatusCode::BAD_REQUEST, status);
    assert_eq!(json!("invalid resource_id: "), body["message"]);

    Ok(())
}

#[tokio::test]
async fn test_search_rejects_invalid_resource_id() -> anyhow::Result<()> {
    let context = test_server::create_test_server_context(
        MockStorageService::new(),
        MockSearcherService::new(),
    );

    let mut body = stubs::fulltext_search_params_json_object();
    body["resource_id"] = json!("not-a-uuid");

    let (status, body) = send_fulltext_search(&context, body).await?;

    assert_eq!(StatusCode::BAD_REQUEST, status);
    assert_eq!(json!("invalid resource_id: not-a-uuid"), body["message"]);

    Ok(())
}

#[tokio::test]
async fn test_search_joins_resource_prefix_and_resolves_index() -> anyhow::Result<()> {
    let mut searcher = MockSearcherService::new();
    searcher
        .expect_search()
        .once()
        .withf(|params, _| {
            params.get_indexes().len() == 1
                && params.get_indexes()[0] == TEST_INDEX_ID
                && params.get_prefix() == Some("res-pre/user-123/inbox")
        })
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let resource_manager =
        StubResourceManager::new(TEST_INDEX_ID.to_string(), "res-pre/".to_string());
    let context = test_server::create_test_server_context_with_resource_manager(
        MockStorageService::new(),
        searcher,
        Arc::new(resource_manager),
    );

    let mut body = stubs::fulltext_search_params_json_object();
    body["prefix"] = json!("user-123/inbox");

    let (status, _) = send_fulltext_search(&context, body).await?;

    assert_eq!(StatusCode::OK, status);

    Ok(())
}
