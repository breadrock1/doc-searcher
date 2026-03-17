use crate::domain::searcher::SearchResult;
use crate::domain::searcher::models::PaginationParams;
use crate::domain::searcher::models::{Pagination, SearchingParams};

/// Trait for performing search operations.
///
/// Defines the interface for search implementations that can handle
/// different types of search requests.
///
/// # Methods
/// * `search` - Performs a search based on provided parameters
///
/// # Arguments
/// * `params` - Configuration parameters for the search operation
///
/// # Returns
/// * `SearchResult<Pagination>` - Paginated search results or error
///
/// # Example
/// ```
/// #[async_trait::async_trait]
/// impl ISearcher for MySearcher {
///     async fn search(&self, params: &SearchingParams) -> SearchResult<Pagination> {
///         // Implementation logic here
///         Ok(Pagination {
///             scroll_id: None,
///             founded: vec![],
///         })
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait ISearcher {
    async fn search(&self, params: &SearchingParams) -> SearchResult<Pagination>;
}

/// Trait for paginating through search results.
///
/// Defines the interface for implementations that handle
/// retrieving subsequent pages of search results.
///
/// # Methods
/// * `paginate` - Retrieves the next page of results using a scroll ID
///
/// # Arguments
/// * `params` - Parameters containing the scroll ID for pagination
///
/// # Returns
/// * `SearchResult<Pagination>` - Next page of search results or error
///
/// # Example
/// ```
/// #[async_trait::async_trait]
/// impl IPaginator for MyPaginator {
///     async fn paginate(&self, params: &PaginationParams) -> SearchResult<Pagination> {
///         // Implementation logic here
///         Ok(Pagination {
///             scroll_id: Some("next_scroll_id".to_string()),
///             founded: next_page_results,
///         })
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait IPaginator {
    async fn paginate(&self, params: &PaginationParams) -> SearchResult<Pagination>;
}
