#[cfg(test)]
pub(crate) mod tests;

pub mod models;

mod error;
pub use error::{SearchError, SearchResult};

mod repository;
pub use repository::{IPaginator, ISearcher};

mod path;
pub use path::{PathError, validate_path};
