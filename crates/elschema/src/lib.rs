pub mod base;
pub mod embeddings;

pub trait ElasticSchema {
    fn build() -> Self;
}
