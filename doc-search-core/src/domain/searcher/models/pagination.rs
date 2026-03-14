use derive_builder::Builder;

use crate::domain::searcher::models::document::FoundedDocument;

/// Represents a paginated search result.
///
/// Contains the search results for a single page along with a scroll ID
/// for retrieving subsequent pages.
///
/// # Fields
/// * `scroll_id` - Identifier for retrieving the next page of results
/// * `founded` - Vector of documents found in the current page
///
/// # Example
/// ```
/// let page = Pagination {
///     scroll_id: Some("scroll_abc123".to_string()),
///     founded: vec![found_doc1, found_doc2],
/// };
/// ```
#[derive(Builder)]
pub struct Pagination {
    pub scroll_id: Option<String>,
    pub founded: Vec<FoundedDocument>,
}

impl Pagination {
    pub fn new(scroll_id: Option<String>, founded: Vec<FoundedDocument>) -> Self {
        Self { scroll_id, founded }
    }
}
