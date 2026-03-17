use derive_builder::Builder;

/// Parameters for creating a new search index.
///
/// # Fields
/// * `id` - Unique identifier for the new index
/// * `knn` - Optional KNN (k-nearest neighbors) index parameters for semantic search support
///
/// # Example
/// ```
/// let create_params = CreateIndexParams {
///     id: "documents_2024".to_string(),
///     knn: Some(KnnIndexParams {
///         knn_dimension: 768,
///         token_limit: 512,
///         overlap_rate: 0.2,
///     }),
/// };
/// ```
#[derive(Debug, Builder)]
pub struct CreateIndexParams {
    pub id: String,
    pub knn: Option<KnnIndexParams>,
}

/// Parameters for configuring a KNN (k-nearest neighbors) index.
///
/// These parameters control how documents are split and embedded
/// for semantic search capabilities.
///
/// # Fields
/// * `knn_dimension` - Dimension of the embedding vectors (e.g., 384 for MiniLM, 768 for BERT)
/// * `token_limit` - Maximum number of tokens per document part/chunk
/// * `overlap_rate` - Overlap rate between consecutive chunks (0.0 to 1.0)
///
/// # Example
/// ```
/// let knn_params = KnnIndexParams {
///     knn_dimension: 768,
///     token_limit: 512,
///     overlap_rate: 0.1, // 10% overlap between chunks
/// };
/// ```
///
/// # Notes
/// - `knn_dimension` must match the embedding model's output dimension
/// - `token_limit` affects granularity of search and storage requirements
/// - `overlap_rate` helps maintain context continuity between chunks
#[derive(Clone, Default, Debug, Builder)]
pub struct KnnIndexParams {
    pub knn_dimension: u32,
    pub token_limit: u32,
    pub overlap_rate: f32,
}
