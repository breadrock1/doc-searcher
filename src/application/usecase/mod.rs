mod searcher;
mod storage;

pub use searcher::SearcherUseCase;
pub use storage::StorageUseCase;

use crate::application::services::tokenizer::TokenizeProvider;

pub type TokenizerBoxed = Box<dyn TokenizeProvider + Send + Sync>;
