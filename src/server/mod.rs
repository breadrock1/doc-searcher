mod config;
pub use config::{CacheConfig, ServerConfig, StorageConfig};

mod error;
pub use error::{ServerError, ServerResult, Success};

pub mod httpserver;

use doc_search_core::application::usecase::searcher::SearcherUseCase;
use doc_search_core::application::usecase::storage::StorageUseCase;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};
use doc_search_core::domain::storage::{IDocumentPartStorage, IIndexStorage};

pub struct ServerApp<Storage, Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone,
{
    storage: StorageUseCase<Storage>,
    searcher: SearcherUseCase<Searcher>,
}

impl<Storage, Searcher> ServerApp<Storage, Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone,
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone,
{
    pub fn new(storage: StorageUseCase<Storage>, searcher: SearcherUseCase<Searcher>) -> Self {
        ServerApp { storage, searcher }
    }

    pub fn get_storage(&self) -> StorageUseCase<Storage> {
        self.storage.clone()
    }

    pub fn get_searcher(&self) -> SearcherUseCase<Searcher> {
        self.searcher.clone()
    }
}
