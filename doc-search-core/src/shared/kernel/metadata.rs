use derive_builder::Builder;

/// Comprehensive metadata for a document (news article)
///
/// Contains all enriched metadata that can be associated with a news article,
/// including geolocation, classifications, subjects, and various labels.
/// This structure is used to store additional information derived from
/// article analysis and processing.
///
/// # Examples
///
/// ```
/// let metadata = DocumentMetadata {
///     pipeline_id: Some(12345),
///     photo: Some("/images/article.jpg".to_string()),
///     source: Some("BBC News".to_string()),
///     semantic_source: Some("International Politics".to_string()),
///     summary: Some("Article discussing recent political developments...".to_string()),
///     locations: vec![
///         DocumentLocation {
///             name: "London".to_string(),
///             latitude: 51.5074,
///             longitude: -0.1278,
///         }
///     ],
///     subjects: vec![DocumentSubject("Politics".to_string())],
///     classes: vec![
///         DocumentClass {
///             name: "News".to_string(),
///             probability: 0.95,
///         }
///     ],
///     icons: vec![DocumentIcon("🏛️".to_string())],
///     groups: vec![DocumentGroup("World News".to_string())],
///     pipelines: vec![PipelineLabel("classification-v2".to_string())],
///     references: vec![DocumentReference("ref-123".to_string())],
/// };
/// ```
#[derive(Clone, Debug, Builder)]
pub struct DocumentMetadata {
    /// Identifier of the pipeline that processed this document
    ///
    /// References the specific processing pipeline version or configuration
    /// used to generate this metadata.
    pub pipeline_id: Option<i64>,

    /// Path to an associated photo or image
    ///
    /// Optional file path or URL pointing to a relevant image for the article.
    pub photo: Option<String>,

    /// Original source of the document
    ///
    /// The publication or origin of the article (e.g., "BBC News", "The Guardian").
    pub source: Option<String>,

    /// Semantic source categorization
    ///
    /// A more abstract categorization of the source's domain or focus area
    /// (e.g., "International Politics", "Technology Reviews").
    pub semantic_source: Option<String>,

    /// Generated summary of the article content
    ///
    /// A brief, automatically generated summary of the article's main points.
    pub summary: Option<String>,

    /// Geographic locations mentioned in or relevant to the article
    ///
    /// List of locations with their names and coordinates that appear in or
    /// are relevant to the article content.
    pub locations: Vec<DocumentLocation>,

    /// Subject tags for the document
    ///
    /// General subject areas or themes covered by the article.
    pub subjects: Vec<DocumentSubject>,

    /// Classified categories with confidence scores
    ///
    /// Machine learning classifications with associated probability scores
    /// indicating the confidence in each classification.
    pub classes: Vec<DocumentClass>,

    /// Icon identifiers for visual representation
    ///
    /// Unicode emoji or icon identifiers that can be used to visually
    /// represent the article's content or category.
    pub icons: Vec<DocumentIcon>,

    /// Group labels for article organization
    ///
    /// Higher-level grouping labels that can be used to organize
    /// articles into broader categories.
    pub groups: Vec<DocumentGroup>,

    /// Pipeline processing labels
    ///
    /// Labels indicating which processing pipelines or algorithms
    /// have been applied to this document.
    pub pipelines: Vec<PipelineLabel>,

    /// External references and identifiers
    ///
    /// References to external systems, documents, or identifiers
    /// associated with this article.
    pub references: Vec<DocumentReference>,
}

/// Icon identifier for visual representation
///
/// A newtype wrapper for icon identifiers, typically Unicode emoji or
/// custom icon identifiers that can be used for visual categorization.
///
/// # Examples
///
/// ```
/// let icon = DocumentIcon("📰".to_string());
/// let tech_icon = DocumentIcon("💻".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct DocumentIcon(pub String);

/// Subject tag for document categorization
///
/// A newtype wrapper for subject labels that describe the main topics
/// or themes covered in the article.
///
/// # Examples
///
/// ```
/// let subject = DocumentSubject("Technology".to_string());
/// let politics = DocumentSubject("Politics".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct DocumentSubject(pub String);

/// External reference identifier
///
/// A newtype wrapper for references to external systems, documents,
/// or identifiers associated with the article.
///
/// # Examples
///
/// ```
/// let reference = DocumentReference("db-id-12345".to_string());
/// let external_ref = DocumentReference("ext:news:789".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct DocumentReference(pub String);

/// Pipeline processing label
///
/// A newtype wrapper for labels indicating which processing pipelines
/// or algorithms have been applied to the document.
///
/// # Examples
///
/// ```
/// let pipeline = PipelineLabel("ner-v2".to_string());
/// let classifier = PipelineLabel("topic-classifier-v3".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct PipelineLabel(pub String);

/// Document group label
///
/// A newtype wrapper for higher-level grouping labels used to organize
/// articles into broader categories or collections.
///
/// # Examples
///
/// ```
/// let group = DocumentGroup("World News".to_string());
/// let feature = DocumentGroup("Featured Stories".to_string());
/// ```
#[derive(Clone, Debug)]
pub struct DocumentGroup(pub String);

/// Geographic location with coordinates
///
/// Represents a geographic location mentioned in or relevant to the article,
/// including its name and precise coordinates for mapping and geospatial analysis.
///
/// # Examples
///
/// ```
/// let location = DocumentLocation {
///     name: "Paris".to_string(),
///     latitude: 48.8566,
///     longitude: 2.3522,
/// };
///
/// let new_york = DocumentLocation {
///     name: "New York City".to_string(),
///     latitude: 40.7128,
///     longitude: -74.0060,
/// };
/// ```
#[derive(Clone, Debug, Builder)]
pub struct DocumentLocation {
    /// Human-readable name of the location
    ///
    /// The common name or place name of the location (e.g., "London", "Eiffel Tower").
    pub name: String,

    /// Latitude coordinate in decimal degrees
    ///
    /// Geographic latitude ranging from -90.0 (South) to 90.0 (North).
    /// Positive values indicate North latitude.
    pub latitude: f64,

    /// Longitude coordinate in decimal degrees
    ///
    /// Geographic longitude ranging from -180.0 (West) to 180.0 (East).
    /// Positive values indicate East longitude.
    pub longitude: f64,
}

/// Document classification with confidence score
///
/// Represents a machine learning classification result for the document,
/// including the classified category and the model's confidence in that
/// classification.
///
/// # Examples
///
/// ```
/// let class = DocumentClass {
///     name: "Sports".to_string(),
///     probability: 0.92,
/// };
///
/// let tech_class = DocumentClass {
///     name: "Technology".to_string(),
///     probability: 0.78,
/// };
/// ```
#[derive(Clone, Debug, Builder)]
pub struct DocumentClass {
    /// Name of the classified category
    ///
    /// The category or class label assigned to the document
    /// (e.g., "Politics", "Sports", "Technology").
    pub name: String,

    /// Confidence probability score
    ///
    /// The model's confidence in this classification, ranging from 0.0 to 1.0.
    /// Higher values indicate greater confidence in the classification.
    pub probability: f64,
}
