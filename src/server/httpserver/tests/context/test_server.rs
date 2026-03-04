use axum::routing::Router;
use std::sync::Arc;
use test_context::AsyncTestContext;

use doc_search::meter::AppMeterRegistry;
use doc_search::server::httpserver::init_server;
use doc_search::server::ServerApp;
use doc_search_core::application::usecase::searcher::SearcherUseCase;
use doc_search_core::application::usecase::storage::StorageUseCase;

use super::super::mocks::searcher::MockSearcherService;
use super::super::mocks::storage::MockStorageService;

const MAX_CONTENT_SIZE: usize = 100;

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
    let meter = AppMeterRegistry::build_local_meter_registry()
        .expect("failed to create local meter registry");

    let searcher_uc = SearcherUseCase::new(Arc::new(searcher));
    let storage_uc = StorageUseCase::new(Arc::new(storage), MAX_CONTENT_SIZE);
    let app = ServerApp::new(Arc::new(storage_uc), Arc::new(searcher_uc), meter);

    let test_server = init_server(app);
    TestServerContext { test_server }
}
