pub mod reranker;
pub mod searcher;
pub mod storage;
pub mod tokenizer;

use rstest::fixture;

pub struct TestEnvironment {
    pub storage: storage::MockStorage,
    pub embeddings: tokenizer::MockEmbeddingsProvider,
    pub reranker: reranker::MockRerankerProvider,
}

#[fixture]
pub fn init_test_environment() -> TestEnvironment {
    TestEnvironment {
        storage: storage::MockStorage::new(),
        embeddings: tokenizer::MockEmbeddingsProvider::new(),
        reranker: reranker::MockRerankerProvider::new(),
    }
}
