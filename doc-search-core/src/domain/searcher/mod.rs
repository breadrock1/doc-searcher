#[cfg(test)]
pub(crate) mod tests;

pub mod models;

mod repository;
pub use repository::{IPaginator, ISearcher};

mod error;
pub use error::{SearchError, SearchResult};
