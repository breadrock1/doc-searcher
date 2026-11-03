use axum::routing::Router;
use std::sync::Arc;
use test_context::AsyncTestContext;

use doc_search::meter::AppMeterRegistry;
use doc_search::server::httpserver::init_server;
use doc_search::server::ServerApp;
use doc_search_core::application::usecase::searcher::SearcherUseCase;
use doc_search_core::application::usecase::storage::StorageUseCase;
use doc_search_core::shared::kernel::Tokens;
use doc_search_core::shared::services::IResourceManager;

use super::super::mocks::reranker::MockRerankerProvider;
use super::super::mocks::resource_manager::stub_resource_manager;
use super::super::mocks::searcher::MockSearcherService;
use super::super::mocks::storage::MockStorageService;
use super::super::mocks::tokenizer::MockEmbeddingsProvider;
use crate::config::ServiceConfig;

const MAX_CONTENT_SIZE: usize = 100;

/// Embeddings provider that always returns empty tokens: enough for the HTTP
/// layer tests, which assert on routing and DTO mapping, not on vector values.
fn stub_embeddings() -> MockEmbeddingsProvider {
    let mut embeddings = MockEmbeddingsProvider::new();
    embeddings
        .expect_compute()
        .returning(|_| Ok(Tokens::default()));
    embeddings
        .expect_compute_bulk()
        .returning(|chunks| Ok(vec![Tokens::default(); chunks.len()]));
    embeddings
}

/// Reranker that keeps the incoming order and scores untouched.
fn stub_reranker() -> MockRerankerProvider {
    let mut reranker = MockRerankerProvider::new();
    reranker.expect_rerank().returning(|_, docs| Ok(docs));
    reranker
}

pub struct TestServerContext {
    pub test_server: Router,
}

impl AsyncTestContext for TestServerContext {
    async fn setup() -> Self {
        create_test_server_context(MockStorageService::new(), MockSearcherService::new())
    }
}

pub fn create_test_server_context(
    storage: MockStorageService,
    searcher: MockSearcherService,
) -> TestServerContext {
    create_test_server_context_with_resource_manager(storage, searcher, stub_resource_manager())
}

pub fn create_test_server_context_with_resource_manager(
    storage: MockStorageService,
    searcher: MockSearcherService,
    resource_manager: Arc<dyn IResourceManager>,
) -> TestServerContext {
    let meter = AppMeterRegistry::build_local_meter_register()
        .expect("failed to create local meter registry");

    let searcher_uc = SearcherUseCase::new(
        Arc::new(searcher),
        Arc::new(stub_embeddings()),
        Arc::new(stub_reranker()),
    );
    let storage_uc = StorageUseCase::new(
        Arc::new(storage),
        Arc::new(stub_embeddings()),
        MAX_CONTENT_SIZE,
    );
    let app = ServerApp::new(
        Arc::new(storage_uc),
        Arc::new(searcher_uc),
        resource_manager,
        meter,
    );

    let config = ServiceConfig::new().expect("load config failed");
    let test_server = init_server(app, config.telemetry());
    TestServerContext { test_server }
}
