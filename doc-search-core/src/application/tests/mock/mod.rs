pub mod searcher;
pub mod storage;
pub mod tokenizer;

use rstest::fixture;

pub struct TestEnvironment {
    pub storage: storage::MockStorage,
    pub embeddings: tokenizer::MockEmbeddingsProvider,
}

#[fixture]
pub fn init_test_environment() -> TestEnvironment {
    TestEnvironment {
        storage: storage::MockStorage::new(),
        embeddings: tokenizer::MockEmbeddingsProvider::new(),
    }
}
