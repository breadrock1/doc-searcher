mod config;
pub use config::{CacheConfig, ServerConfig, StorageConfig};

mod error;
pub use error::{ServerError, ServerResult, Success};

pub mod httpserver;

use doc_search_core::application::usecase::searcher::SearcherUseCase;
use doc_search_core::application::usecase::storage::StorageUseCase;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};
use doc_search_core::shared::services::IResourceManager;
use std::sync::Arc;

use crate::meter::AppMeterRegistry;

pub struct ServerApp<Storage, Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync,
{
    storage: Arc<StorageUseCase<Storage>>,
    searcher: Arc<SearcherUseCase<Searcher>>,
    resource_manager: Arc<dyn IResourceManager>,
    meter_handle: Arc<AppMeterRegistry>,
}

impl<Storage, Searcher> ServerApp<Storage, Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync,
{
    pub fn new(
        storage: Arc<StorageUseCase<Storage>>,
        searcher: Arc<SearcherUseCase<Searcher>>,
        resource_manager: Arc<dyn IResourceManager>,
        meter_handle: Arc<AppMeterRegistry>,
    ) -> Self {
        ServerApp {
            storage,
            searcher,
            resource_manager,
            meter_handle,
        }
    }

    pub fn get_storage(&self) -> Arc<StorageUseCase<Storage>> {
        self.storage.clone()
    }

    pub fn get_searcher(&self) -> Arc<SearcherUseCase<Searcher>> {
        self.searcher.clone()
    }

    pub fn get_resource_manager(&self) -> Arc<dyn IResourceManager> {
        self.resource_manager.clone()
    }

    pub fn get_meter_handle(&self) -> Arc<AppMeterRegistry> {
        self.meter_handle.clone()
    }
}
