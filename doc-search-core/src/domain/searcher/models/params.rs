use derive_builder::Builder;
use std::fmt::Display;

/// Type alias for a collection of search indexes.
///
/// Represents multiple indexes to search across.
pub type SearchIndexes = Vec<String>;

/// Parameters for configuring a search request.
///
/// This structure encapsulates all parameters needed to perform a search,
/// including which indexes to search, what type of search to perform,
/// pagination settings, and filters.
///
/// # Fields
/// * `indexes` - List of indexes to search in
/// * `kind` - Type of search to perform (retrieve, full-text, semantic, or hybrid)
/// * `result` - Pagination and result formatting parameters
/// * `filter` - Optional filters to narrow down results
pub struct SearchingParams {
    indexes: SearchIndexes,
    kind: SearchKindParams,
    result: ResultParams,
    filter: Option<FilterParams>,
}

impl SearchingParams {
    pub fn new(
        indexes: Vec<String>,
        kind: SearchKindParams,
        result: ResultParams,
        filter: Option<FilterParams>,
    ) -> Self {
        Self {
            indexes,
            kind,
            result,
            filter,
        }
    }

    pub fn get_indexes(&self) -> &[String] {
        self.indexes.as_slice()
    }

    pub fn get_kind(&self) -> &SearchKindParams {
        &self.kind
    }

    pub fn get_result(&self) -> &ResultParams {
        &self.result
    }

    pub fn get_filter(&self) -> Option<&FilterParams> {
        self.filter.as_ref()
    }
}

/// Enum representing different types of search operations.
///
/// # Variants
/// * `Retrieve` - Direct retrieval of documents by parameters
/// * `FullText` - Traditional full-text search with query string
/// * `Semantic` - Vector-based semantic similarity search
/// * `Hybrid` - Combination of full-text and semantic search
#[derive(Debug)]
pub enum SearchKindParams {
    Retrieve(RetrieveIndexDocumentsParams),
    FullText(FullTextSearchingParams),
    Semantic(SemanticSearchingParams),
    Hybrid(HybridSearchingParams),
}

impl Display for SearchKindParams {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let kind = match &self {
            SearchKindParams::Retrieve(_) => "retrieve",
            SearchKindParams::FullText(_) => "fulltext",
            SearchKindParams::Semantic(_) => "semantic",
            SearchKindParams::Hybrid(_) => "hybrid",
        };
        write!(fmt, "{}", kind)
    }
}

/// Parameters for directly retrieving documents from an index.
///
/// # Fields
/// * `path` - Optional file path to filter documents by
#[derive(Debug, Builder)]
pub struct RetrieveIndexDocumentsParams {
    pub path: Option<String>,
}

/// Parameters for full-text search operations.
///
/// # Fields
/// * `query` - Optional search query string
#[derive(Debug, Builder)]
pub struct FullTextSearchingParams {
    pub query: Option<String>,
}

/// Parameters for semantic (vector-based) search operations.
///
/// # Fields
/// * `query` - Search query text
/// * `knn_amount` - Number of nearest neighbors to return
/// * `min_score` - Minimum similarity score threshold (optional)
/// * `model_id` - Identifier of the embedding model to use (optional)
/// * `tokens` - Pre-computed embedding tokens (optional, for optimization)
///
/// # Example
/// ```
/// let semantic_params = SemanticSearchingParams {
///     query: "machine learning".to_string(),
///     knn_amount: 10,
///     min_score: Some(0.7),
///     model_id: Some("bert-base".to_string()),
///     tokens: None,
/// };
/// ```
#[derive(Debug, Builder)]
pub struct SemanticSearchingParams {
    pub query: String,
    pub knn_amount: u16,
    pub min_score: Option<f32>,
    pub model_id: Option<String>,
    pub tokens: Option<Vec<f64>>,
}

/// Parameters for hybrid search combining full-text and semantic search.
///
/// # Fields
/// * `query` - Search query text
/// * `knn_amount` - Number of nearest neighbors for semantic component
/// * `min_score` - Minimum combined score threshold (optional)
/// * `model_id` - Identifier of the embedding model to use (optional)
#[derive(Debug, Builder)]
pub struct HybridSearchingParams {
    pub query: String,
    pub knn_amount: u16,
    pub min_score: Option<f32>,
    pub model_id: Option<String>,
}

/// Parameters for paginating through search results.
///
/// # Fields
/// * `scroll_id` - Scroll ID from previous search results for continuation
#[derive(Debug, Builder)]
pub struct PaginationParams {
    pub scroll_id: String,
}

/// Filter parameters for narrowing down search results.
///
/// All fields are optional to allow flexible filter combinations.
///
/// # Fields
/// * `doc_part_id` - Filter by specific document part number
/// * `size_from` - Minimum file size in bytes
/// * `size_to` - Maximum file size in bytes
/// * `created_from` - Minimum creation timestamp
/// * `created_to` - Maximum creation timestamp
/// * `modified_from` - Minimum modification timestamp
/// * `modified_to` - Maximum modification timestamp
/// * `pipeline_id` - Filter by processing pipeline ID
/// * `source` - Filter by document source
/// * `semantic_source` - Filter by semantic source type
/// * `distance` - Filter by geographic distance
/// * `location_coords` - Filter by geographic coordinates [longitude, latitude]
/// * `doc_class` - Filter by document classification
/// * `doc_class_probability` - Filter by classification probability threshold
///
/// # Example
/// ```
/// let filters = FilterParams {
///     doc_part_id: Some(1),
///     size_from: Some(1024),
///     size_to: Some(1048576),
///     created_from: Some(1634567890),
///     created_to: Some(1634568890),
///     modified_from: None,
///     modified_to: None,
///     pipeline_id: Some(42),
///     source: Some("upload".to_string()),
///     semantic_source: None,
///     distance: None,
///     location_coords: None,
///     doc_class: None,
///     doc_class_probability: None,
/// };
/// ```
#[derive(Clone, Debug, Builder)]
pub struct FilterParams {
    pub doc_part_id: Option<usize>,
    pub size_from: Option<u32>,
    pub size_to: Option<u32>,
    pub created_from: Option<i64>,
    pub created_to: Option<i64>,
    pub modified_from: Option<i64>,
    pub modified_to: Option<i64>,
    #[builder(default)]
    pub pipeline_id: Option<i64>,
    #[builder(default)]
    pub source: Option<String>,
    #[builder(default)]
    pub semantic_source: Option<String>,
    #[builder(default)]
    pub distance: Option<String>,
    #[builder(default)]
    pub location_coords: Option<Vec<f64>>,
    #[builder(default)]
    pub doc_class: Option<String>,
    #[builder(default)]
    pub doc_class_probability: Option<f64>,
}

/// Parameters for controlling result pagination and formatting.
///
/// # Fields
/// * `size` - Number of results per page
/// * `offset` - Number of results to skip (for offset-based pagination)
/// * `order` - Sort order (ASC or DESC)
/// * `highlight_items` - Maximum number of highlight fragments per document
/// * `highlight_item_size` - Maximum size of each highlight fragment
/// * `include_extra_fields` - Whether to include additional metadata fields
///
/// # Example
/// ```
/// let result_params = ResultParams {
///     size: 20,
///     offset: 0,
///     order: ResultOrder::DESC,
///     highlight_items: Some(3),
///     highlight_item_size: Some(100),
///     include_extra_fields: Some(true),
/// };
/// ```
#[derive(Clone, Default, Debug, Builder)]
pub struct ResultParams {
    pub size: i64,
    pub offset: i64,
    pub order: ResultOrder,
    pub highlight_items: Option<u16>,
    pub highlight_item_size: Option<u32>,
    pub include_extra_fields: Option<bool>,
}

/// Sort order for search results.
///
/// # Variants
/// * `ASC` - Ascending order (oldest/smallest first)
/// * `DESC` - Descending order (newest/largest first) - default
#[derive(Clone, Debug, Default)]
pub enum ResultOrder {
    ASC,
    #[default]
    DESC,
}
