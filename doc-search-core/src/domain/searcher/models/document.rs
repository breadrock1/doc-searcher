use derive_builder::Builder;

use crate::shared::kernel::metadata::DocumentMetadata;

/// Represents a document found in search results.
///
/// This structure contains all information about a document that matched the search criteria,
/// including its identification, relevance score, and complete content.
///
/// # Fields
/// * `id` - Unique identifier of the document
/// * `index` - Name of the search index where the document was found
/// * `score` - Relevance score of the document (optional, used in full-text search)
/// * `highlight` - Vector of text fragments with highlighted matching terms
/// * `document` - Complete document content and metadata
///
/// # Example
/// ```
/// let found_doc = FoundedDocument {
///     id: "doc123".to_string(),
///     index: "documents".to_string(),
///     score: Some(0.95),
///     highlight: vec!["<em>search</em> term".to_string()],
///     document: document_part,
/// };
/// ```
#[derive(Clone, Debug, Builder)]
pub struct FoundedDocument {
    pub id: String,
    pub index: String,
    pub score: Option<f64>,
    pub highlight: Vec<String>,
    pub document: DocumentPartEntrails,
}

/// Represents the detailed content and metadata of a document part.
///
/// This structure holds comprehensive information about a specific part of a larger document,
/// including file properties, content, embeddings, and metadata.
///
/// # Fields
/// * `large_doc_id` - Identifier of the parent document
/// * `doc_part_id` - Sequential number of this part within the parent document
/// * `file_name` - Name of the file
/// * `file_path` - Full path to the file in the storage system
/// * `file_size` - Size of the file in bytes
/// * `created_at` - Unix timestamp of file creation
/// * `modified_at` - Unix timestamp of last modification
/// * `content` - Raw text content of the document part (optional)
/// * `chunked_text` - Text split into chunks for processing (optional)
/// * `embeddings` - Vector embeddings for semantic search (optional)
/// * `metadata` - Additional document metadata (optional)
///
/// # Example
/// ```
/// let doc_part = DocumentPartEntrails {
///     large_doc_id: "parent123".to_string(),
///     doc_part_id: 1,
///     file_name: "document.pdf".to_string(),
///     file_path: "/storage/docs/document.pdf".to_string(),
///     file_size: 1024,
///     created_at: 1634567890,
///     modified_at: 1634567890,
///     content: Some("Document content...".to_string()),
///     chunked_text: None,
///     embeddings: None,
///     metadata: None,
/// };
/// ```
#[derive(Clone, Default, Debug, Builder)]
pub struct DocumentPartEntrails {
    pub large_doc_id: String,
    pub doc_part_id: usize,
    pub file_name: String,
    pub file_path: String,
    pub file_size: u32,
    pub created_at: i64,
    pub modified_at: i64,
    pub content: Option<String>,
    pub chunked_text: Option<Vec<String>>,
    pub embeddings: Option<Vec<Embeddings>>,
    pub metadata: Option<DocumentMetadata>,
}

/// Represents vector embeddings for semantic search.
///
/// Contains the numerical representation of text for similarity calculations.
///
/// # Fields
/// * `knn` - Vector of floating-point numbers representing the text embedding
///
/// # Example
/// ```
/// let embedding = Embeddings {
///     knn: vec![0.123, 0.456, 0.789, -0.123],
/// };
/// ```
#[derive(Clone, Debug)]
pub struct Embeddings {
    pub knn: Vec<f64>,
}

impl From<Vec<f64>> for Embeddings {
    fn from(tokens: Vec<f64>) -> Self {
        Embeddings { knn: tokens }
    }
}
