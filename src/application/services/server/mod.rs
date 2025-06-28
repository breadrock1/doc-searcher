mod error;

pub use error::{ServerError, ServerResult, Success};

use crate::application::services::storage::{DocumentManager, DocumentSearcher, IndexManager, PaginateManager};
use crate::application::usecase::{SearcherUseCase, StorageUseCase};

pub struct ServerApp<Storage, Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    storage: StorageUseCase<Storage>,
    searcher: SearcherUseCase<Searcher>,
}

impl<Storage, Searcher> ServerApp<Storage, Searcher>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
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
