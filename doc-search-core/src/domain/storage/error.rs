use thiserror::Error;

/// Type alias for storage operation results.
///
/// Simplifies function signatures throughout the storage module
/// by providing a consistent return type.
pub type StorageResult<T> = Result<T, StorageError>;

/// Represents possible errors that can occur during storage operations.
///
/// This enum provides comprehensive error categorization for all storage-related operations,
/// including authentication, connection issues, document operations, and validation failures.
/// Each variant wraps an `anyhow::Error` to preserve context and backtrace information.
///
/// # Variants
/// * `AuthenticationFailed` - Authentication failure with storage service
/// * `ConnectionError` - Network or connection issues
/// * `IndexNotFound` - Requested index does not exist
/// * `DocumentNotFound` - Requested document or document part not found
/// * `DocumentAlreadyExists` - Attempt to store document that already exists
/// * `CantSplitLargeDocuments` - Error during document splitting process
/// * `ValidationError` - Invalid parameters or document data
/// * `InternalError` - Internal system error during storage operation
/// * `UnknownError` - Catch-all for unexpected errors
///
/// # Display Format
/// Each variant follows a specific format for clear error identification:
/// * Authentication: "storage: auth failed: {0}"
/// * Connection: "storage: connection error: {0}"
/// * Index: "storage: index has not been found: {0}"
/// * Document: "storage: document has not been found: {0}"
/// * Document exists: "storage: document already exists: {0}"
/// * Split error: "can't split large document: {0}"
/// * Validation: "storage: validation error: {0}"
/// * Internal: "storage: internal error: {0}"
/// * Unknown: "storage: unknown error: {0}"
///
/// # Example
/// ```
/// fn validate_document(doc: &LargeDocument) -> Result<(), StorageError> {
///     if doc.content.is_empty() {
///         return Err(StorageError::ValidationError(
///             anyhow::anyhow!("Document content cannot be empty")
///         ));
///     }
///
///     if doc.file_size == 0 {
///         return Err(StorageError::ValidationError(
///             anyhow::anyhow!("File size must be greater than 0")
///         ));
///     }
///
///     Ok(())
/// }
///
/// async fn store_document() -> Result<(), StorageError> {
///     // Simulate connection error
///     if !is_connected().await {
///         return Err(StorageError::ConnectionError(
///             anyhow::anyhow!("Failed to connect to storage service at localhost:9200")
///         ));
///     }
///
///     // Simulate document not found
///     Err(StorageError::DocumentNotFound(
///         anyhow::anyhow!("Document with ID 'doc_123' not found in index 'documents'")
///     ))
/// }
/// ```
///
/// # Usage in Error Handling
/// ```
/// async fn process_document_storage(
///     storage: &dyn IDocumentPartStorage,
///     index: &str,
///     parts: AllDocumentParts,
/// ) -> Result<StoredDocumentPartsInfo, StorageError> {
///     // Validate input
///     if parts.is_empty() {
///         return Err(StorageError::ValidationError(
///             anyhow::anyhow!("Cannot store empty document parts collection")
///         ));
///     }
///
///     // Check if document already exists
///     let large_doc_id = &parts[0].large_doc_id;
///     match storage.get_document_parts(index, large_doc_id).await {
///         Ok(_) => {
///             return Err(StorageError::DocumentAlreadyExists(
///                 anyhow::anyhow!("Document {} already exists in index {}", large_doc_id, index)
///             ));
///         }
///         Err(StorageError::DocumentNotFound(_)) => {
///             // Expected - document doesn't exist, proceed with storage
///         }
///         Err(e) => return Err(e), // Propagate other errors
///     }
///
///     // Store the document parts
///     storage.store_document_parts(index, parts).await
/// }
/// ```
///
/// # Error Conversion Pattern
/// ```
/// impl From<elasticsearch::Error> for StorageError {
///     fn from(err: elasticsearch::Error) -> Self {
///         match err {
///             elasticsearch::Error::Authentication(_) =>
///                 StorageError::AuthenticationFailed(anyhow::anyhow!(err)),
///             elasticsearch::Error::Connection(_) =>
///                 StorageError::ConnectionError(anyhow::anyhow!(err)),
///             elasticsearch::Error::NotFound(_) =>
///                 StorageError::DocumentNotFound(anyhow::anyhow!(err)),
///             elasticsearch::Error::IndexNotFound(_) =>
///                 StorageError::IndexNotFound(anyhow::anyhow!(err)),
///             _ => StorageError::UnknownError(anyhow::anyhow!(err)),
///         }
///     }
/// }
/// ```
#[derive(Debug, Error)]
pub enum StorageError {
    /// Authentication failed with the search service.
    ///
    /// This error occurs when:
    /// * Invalid API keys or tokens are provided
    /// * Credentials have expired
    /// * Insufficient permissions for the requested operation
    ///
    /// # Example
    /// ```
    /// Err(StorageError::AuthenticationFailed(
    ///     anyhow::anyhow!("API key has expired")
    /// ))
    /// ```
    #[error("storage: auth failed: {0}")]
    AuthenticationFailed(anyhow::Error),

    /// Connection error when communicating with the storage service.
    ///
    /// This error occurs when:
    /// * Network connectivity issues
    /// * Service endpoint is unreachable
    /// * Timeout during connection establishment
    /// * DNS resolution failures
    ///
    /// # Example
    /// ```
    /// Err(StorageError::ConnectionError(
    ///     anyhow::anyhow!("Failed to connect to storage service at localhost:9200")
    /// ))
    /// ```
    #[error("storage: connection error: {0}")]
    ConnectionError(anyhow::Error),

    /// The requested index was not found.
    ///
    /// This error occurs when:
    /// * The specified index name doesn't exist
    /// * Index has been deleted or is unavailable
    /// * Incorrect index name was provided
    ///
    /// # Example
    /// ```
    /// Err(StorageError::IndexNotFound(
    ///     anyhow::anyhow!("Index 'documents_2023' does not exist")
    /// ))
    /// ```
    #[error("storage: index has not been founded: {0}")]
    IndexNotFound(anyhow::Error),

    /// The requested document was not found.
    ///
    /// This error occurs when:
    /// * The specified document doesn't exist
    /// * Index has been deleted or is unavailable
    /// * Incorrect document id was provided
    ///
    /// # Example
    /// ```
    /// Err(StorageError::DocumentNotFound(
    ///     anyhow::anyhow!("Document with id 'documents_2023' does not exist")
    /// ))
    /// ```
    #[error("storage: document has not been founded: {0}")]
    DocumentNotFound(anyhow::Error),

    /// The requested document has been already stored into index.
    ///
    /// This error occurs when:
    /// * The specified document with id already exists
    /// * Incorrect index or document id was provided
    ///
    /// # Example
    /// ```
    /// Err(StorageError::DocumentAlreadyExists(
    ///     anyhow::anyhow!("Index 'documents_2023' does not exist")
    /// ))
    /// ```
    #[error("storage: document already exists: {0}")]
    DocumentAlreadyExists(anyhow::Error),

    /// The requested document has not been split on document parts.
    ///
    /// This error occurs when:
    /// * The specified document with empty content
    /// * Failed to build document part object
    ///
    /// # Example
    /// ```
    /// Err(StorageError::DocumentAlreadyExists(
    ///     anyhow::anyhow!("Index 'documents_2023' does not exist")
    /// ))
    /// ```
    #[error("can't split large document: {0}")]
    CantSplitLargeDocuments(anyhow::Error),

    /// Validation error parameters to store document.
    ///
    /// This error occurs when:
    /// * Required parameters are missing
    /// * Parameter values are out of valid range
    /// * Invalid combinations of parameters
    /// * Malformed query syntax
    ///
    /// # Example
    /// ```
    /// Err(StorageError::ValidationError(
    ///     anyhow::anyhow!("missing fields into structure")
    /// ))
    /// ```
    #[error("storage: validation error: {0}")]
    ValidationError(anyhow::Error),

    /// Internal system error during storing.
    ///
    /// This error occurs when:
    /// * Resource allocation fails
    /// * Internal data structures are corrupted
    /// * Unexpected system state
    /// * Configuration errors
    ///
    /// # Example
    /// ```
    /// Err(StorageError::InternalError(
    ///     anyhow::anyhow!("Failed to allocate memory for storing results")
    /// ))
    /// ```
    #[error("storage: internal error: {0}")]
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
    /// Err(StorageError::UnknownError(
    ///     anyhow::anyhow!("Unexpected response format from storage service")
    /// ))
    /// ```
    #[error("storage: unknown error: {0}")]
    UnknownError(anyhow::Error),
}
