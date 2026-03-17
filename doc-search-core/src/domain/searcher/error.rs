use thiserror::Error;

/// Type alias for search operation results.
///
/// This alias simplifies function signatures throughout the search module
/// by providing a consistent return type for all search operations.
///
/// # Example
/// ```
/// type SearchResult<T> = Result<T, SearchError>;
///
/// async fn search_documents(query: &str) -> SearchResult<Vec<FoundedDocument>> {
///     // Implementation
/// }
/// ```
pub type SearchResult<T> = Result<T, SearchError>;

/// Represents possible errors that can occur during search operations.
///
/// This enum provides comprehensive error categorization for all search-related operations,
/// including authentication, connection issues, validation failures, and service-level errors.
/// Each variant wraps an `anyhow::Error` to preserve context and backtrace information.
///
/// # Variants
/// * `AuthenticationFailed` - Occurs when authentication with the search service fails
/// * `ConnectionError` - Happens when unable to establish or maintain connection to search service
/// * `IndexNotFound` - Indicates that the specified search index does not exist
/// * `ValidationError` - Occurs when search parameters fail validation rules
/// * `ServiceError` - Represents errors returned by the search service itself
/// * `InternalError` - Covers internal system errors during search processing
/// * `UnknownError` - Catch-all for unexpected or uncategorized errors
///
/// # Display Format
/// Each variant follows the format: "searcher: [error type]: [detailed error message]"
///
/// # Example
/// ```
/// fn perform_search() -> Result<(), SearchError> {
///     // Simulate authentication failure
///     Err(SearchError::AuthenticationFailed(anyhow::anyhow!("Invalid API key")))
/// }
///
/// // Error output: "searcher: auth failed: Invalid API key"
///
/// fn validate_query() -> Result<(), SearchError> {
///     // Simulate validation error
///     Err(SearchError::ValidationError(anyhow::anyhow!("Query too short")))
/// }
///
/// // Error output: "searcher: validation error: Query too short"
/// ```
///
/// # Usage in Error Handling
/// ```
/// use anyhow::Context;
///
/// async fn search_documents() -> Result<Vec<FoundedDocument>, SearchError> {
///     let client = get_search_client()
///         .await
///         .map_err(|e| SearchError::ConnectionError(anyhow::anyhow!(e)))?;
///
///     let results = client
///         .search("query")
///         .await
///         .map_err(|e| match e.code() {
///             401 => SearchError::AuthenticationFailed(anyhow::anyhow!(e)),
///             404 => SearchError::IndexNotFound(anyhow::anyhow!(e)),
///             _ => SearchError::ServiceError(anyhow::anyhow!(e)),
///         })?;
///
///     Ok(results)
/// }
/// ```
///
/// # Error Propagation Pattern
/// ```
/// fn process_search_params(params: &SearchingParams) -> Result<(), SearchError> {
///     if params.indexes.is_empty() {
///         return Err(SearchError::ValidationError(
///             anyhow::anyhow!("At least one index must be specified")
///         ));
///     }
///
///     // Validate search kind specific parameters
///     match &params.kind {
///         SearchKindParams::Semantic(semantic_params) => {
///             if semantic_params.knn_amount == 0 {
///                 return Err(SearchError::ValidationError(
///                     anyhow::anyhow!("knn_amount must be greater than 0")
///                 ));
///             }
///         },
///         SearchKindParams::FullText(text_params) => {
///             if text_params.query.as_ref().map(|q| q.is_empty()).unwrap_or(true) {
///                 return Err(SearchError::ValidationError(
///                     anyhow::anyhow!("Query cannot be empty")
///                 ));
///             }
///         },
///         _ => {} // Other variants don't need validation
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Error)]
pub enum SearchError {
    /// Authentication failed with the search service.
    ///
    /// This error occurs when:
    /// * Invalid API keys or tokens are provided
    /// * Credentials have expired
    /// * Insufficient permissions for the requested operation
    ///
    /// # Example
    /// ```
    /// Err(SearchError::AuthenticationFailed(
    ///     anyhow::anyhow!("API key has expired")
    /// ))
    /// ```
    #[error("searcher: auth failed: {0}")]
    AuthenticationFailed(anyhow::Error),

    /// Connection error when communicating with the search service.
    ///
    /// This error occurs when:
    /// * Network connectivity issues
    /// * Service endpoint is unreachable
    /// * Timeout during connection establishment
    /// * DNS resolution failures
    ///
    /// # Example
    /// ```
    /// Err(SearchError::ConnectionError(
    ///     anyhow::anyhow!("Failed to connect to search service at localhost:9200")
    /// ))
    /// ```
    #[error("searcher: auth failed: {0}")]
    ConnectionError(anyhow::Error),

    /// The requested search index was not found.
    ///
    /// This error occurs when:
    /// * The specified index name doesn't exist
    /// * Index has been deleted or is unavailable
    /// * Incorrect index name was provided
    ///
    /// # Example
    /// ```
    /// Err(SearchError::IndexNotFound(
    ///     anyhow::anyhow!("Index 'documents_2023' does not exist")
    /// ))
    /// ```
    #[error("searcher: index has not been founded: {0}")]
    IndexNotFound(anyhow::Error),

    /// Validation error in search parameters.
    ///
    /// This error occurs when:
    /// * Required parameters are missing
    /// * Parameter values are out of valid range
    /// * Invalid combinations of parameters
    /// * Malformed query syntax
    ///
    /// # Example
    /// ```
    /// Err(SearchError::ValidationError(
    ///     anyhow::anyhow!("knn_amount must be between 1 and 100, got 0")
    /// ))
    /// ```
    #[error("searcher: validation error: {0}")]
    ValidationError(anyhow::Error),

    /// Error returned by the search service itself.
    ///
    /// This error occurs when:
    /// * Service returns an error response
    /// * Query execution fails on the service side
    /// * Service-specific constraints are violated
    ///
    /// # Example
    /// ```
    /// Err(SearchError::ServiceError(
    ///     anyhow::anyhow!("Search service returned: query timeout after 30s")
    /// ))
    /// ```
    #[error("searcher: returned error into response: {0}")]
    ServiceError(anyhow::Error),

    /// Internal system error during search processing.
    ///
    /// This error occurs when:
    /// * Resource allocation fails
    /// * Internal data structures are corrupted
    /// * Unexpected system state
    /// * Configuration errors
    ///
    /// # Example
    /// ```
    /// Err(SearchError::InternalError(
    ///     anyhow::anyhow!("Failed to allocate memory for search results")
    /// ))
    /// ```
    #[error("searcher: internal error: {0}")]
    InternalError(anyhow::Error),

    /// Unknown or uncategorized error.
    ///
    /// This error serves as a catch-all for:
    /// * Unexpected error types
    /// * Errors that don't fit other categories
    /// * Placeholder during development
    ///
    /// # Example
    /// ```
    /// Err(SearchError::UnknownError(
    ///     anyhow::anyhow!("Unexpected response format from search service")
    /// ))
    /// ```
    #[error("searcher: unknown error: {0}")]
    UnknownError(anyhow::Error),
}
