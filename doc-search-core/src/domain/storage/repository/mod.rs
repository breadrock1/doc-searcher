use crate::domain::storage::StorageResult;
use crate::domain::storage::models::{AllDocumentParts, DocumentPart, StoredDocumentPartsInfo};
use crate::domain::storage::models::{CreateIndexParams, IndexId};

/// Trait for managing search index lifecycle operations.
///
/// Provides asynchronous methods for creating, deleting, and retrieving
/// search indexes in the storage system.
///
/// # Methods
/// * `create_index` - Creates a new search index with specified parameters
/// * `delete_index` - Removes an existing search index and all its contents
/// * `get_index` - Retrieves information about a specific index
/// * `get_all_indexes` - Lists all available indexes in the storage system
///
/// # Arguments
/// * `create_index`:
///   - `index` - Parameters for index creation including ID and KNN settings
/// * `delete_index`:
///   - `id` - Identifier of the index to delete
/// * `get_index`:
///   - `id` - Identifier of the index to retrieve
/// * `get_all_indexes`:
///   - No arguments
///
/// # Returns
/// * `create_index` - `StorageResult<IndexId>` - ID of the created index
/// * `delete_index` - `StorageResult<()>` - Empty result on success
/// * `get_index` - `StorageResult<IndexId>` - Index information
/// * `get_all_indexes` - `StorageResult<Vec<IndexId>>` - List of index IDs
///
/// # Example
/// ```
/// #[async_trait::async_trait]
/// impl IIndexStorage for ElasticsearchStorage {
///     async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<IndexId> {
///         // Implementation for creating index in Elasticsearch
///         Ok(index.id.clone())
///     }
///
///     async fn delete_index(&self, id: &str) -> StorageResult<()> {
///         // Implementation for deleting index
///         Ok(())
///     }
///
///     async fn get_index(&self, id: &str) -> StorageResult<IndexId> {
///         // Implementation for retrieving index info
///         Ok(id.to_string())
///     }
///
///     async fn get_all_indexes(&self) -> StorageResult<Vec<IndexId>> {
///         // Implementation for listing all indexes
///         Ok(vec!["index1".to_string(), "index2".to_string()])
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait IIndexStorage {
    async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<IndexId>;
    async fn delete_index(&self, id: &str) -> StorageResult<()>;
    async fn get_index(&self, id: &str) -> StorageResult<IndexId>;
    async fn get_all_indexes(&self) -> StorageResult<Vec<IndexId>>;
}

/// Trait for managing document part storage operations.
///
/// Provides asynchronous methods for storing, retrieving, and deleting
/// document parts within search indexes.
///
/// # Methods
/// * `store_document_parts` - Stores multiple document parts in an index
/// * `get_document_parts` - Retrieves all parts of a large document
/// * `get_document_part` - Retrieves a single specific document part
/// * `delete_document_parts` - Deletes all parts of a large document
///
/// # Arguments
/// * `store_document_parts`:
///   - `index` - Target index name
///   - `all_doc_parts` - Complete collection of document parts to store
/// * `get_document_parts`:
///   - `index` - Index to search in
///   - `large_doc_id` - ID of the large document whose parts to retrieve
/// * `get_document_part`:
///   - `index` - Index to search in
///   - `doc_part_id` - ID of the specific document part to retrieve
/// * `delete_document_parts`:
///   - `index` - Index containing the document
///   - `large_doc_id` - ID of the large document whose parts to delete
///
/// # Returns
/// * `store_document_parts` - `StorageResult<StoredDocumentPartsInfo>` - Information about stored parts
/// * `get_document_parts` - `StorageResult<AllDocumentParts>` - Collection of document parts
/// * `get_document_part` - `StorageResult<DocumentPart>` - Single document part
/// * `delete_document_parts` - `StorageResult<()>` - Empty result on success
///
/// # Example
/// ```
/// #[async_trait::async_trait]
/// impl IDocumentPartStorage for ElasticsearchStorage {
///     async fn store_document_parts(
///         &self,
///         index: &str,
///         all_doc_parts: AllDocumentParts,
///     ) -> StorageResult<StoredDocumentPartsInfo> {
///         // Implementation for bulk indexing document parts
///         Ok(StoredDocumentPartsInfo {
///             large_doc_id: all_doc_parts[0].large_doc_id.clone(),
///             first_part_id: format!("{}_part_1", all_doc_parts[0].large_doc_id),
///             doc_parts_amount: all_doc_parts.len(),
///         })
///     }
///
///     async fn get_document_parts(
///         &self,
///         index: &str,
///         large_doc_id: &str,
///     ) -> StorageResult<AllDocumentParts> {
///         // Implementation for retrieving all parts of a document
///         Ok(vec![])
///     }
///
///     async fn get_document_part(
///         &self,
///         index: &str,
///         doc_part_id: &str,
///     ) -> StorageResult<DocumentPart> {
///         // Implementation for retrieving a single part
///         Err(StorageError::DocumentNotFound(
///             anyhow::anyhow!("Document part not found: {}", doc_part_id)
///         ))
///     }
///
///     async fn delete_document_parts(
///         &self,
///         index: &str,
///         large_doc_id: &str,
///     ) -> StorageResult<()> {
///         // Implementation for deleting document parts
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait IDocumentPartStorage {
    async fn store_document_parts(
        &self,
        index: &str,
        all_doc_parts: AllDocumentParts,
    ) -> StorageResult<StoredDocumentPartsInfo>;

    async fn get_document_parts(
        &self,
        index: &str,
        large_doc_id: &str,
    ) -> StorageResult<AllDocumentParts>;

    async fn get_document_part(
        &self,
        index: &str,
        doc_part_id: &str,
    ) -> StorageResult<DocumentPart>;
    async fn delete_document_parts(&self, index: &str, large_doc_id: &str) -> StorageResult<()>;
}
