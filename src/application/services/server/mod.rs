mod error;

pub use error::{ServerError, ServerResult, Success};

use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::application::services::tokenizer::TokenizeProvider;
use crate::application::usecase::{SearcherUseCase, StorageUseCase};

pub struct ServerApp<Storage, Searcher, Tokenizer>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
    Tokenizer: TokenizeProvider + Send + Sync + Clone,
{
    storage: StorageUseCase<Storage, Tokenizer>,
    searcher: SearcherUseCase<Searcher, Tokenizer>,
}

impl<Storage, Searcher, Tokenizer> ServerApp<Storage, Searcher, Tokenizer>
where
    Searcher: DocumentSearcher + PaginateManager + Send + Sync + Clone,
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
    Tokenizer: TokenizeProvider + Send + Sync + Clone,
{
    pub fn new(
        storage: StorageUseCase<Storage, Tokenizer>,
        searcher: SearcherUseCase<Searcher, Tokenizer>,
    ) -> Self {
        ServerApp { storage, searcher }
    }

    pub fn get_storage(&self) -> StorageUseCase<Storage, Tokenizer> {
        self.storage.clone()
    }

    pub fn get_searcher(&self) -> SearcherUseCase<Searcher, Tokenizer> {
        self.searcher.clone()
    }
}
