mod pagination;
pub mod params;

pub const QUERY_FIELD_VALUE: &str = "query string";
pub const EMBEDDINGS_MODEL_ID: &str = "bge-model";
pub const CURRENT_TIMESTAMP: i64 = 1756498133;
pub const PIPELINE_ID_FILTER_PARAMS: i64 = 1;
pub const SOURCE_FILTER_PARAMS: &str = "source";
pub const SEMANTIC_SOURCE_FILTER_PARAMS: &str = "semantic-source";
pub const DISTANCE_FILTER_PARAMS: &str = "10km";
pub const LOCATION_COORDS_FILTER_PARAMS: &[f64; 2] = &[40.0, 25.0];
pub const DOCUMENT_CLASS_FILTER_PARAMS: &str = "class";
pub const DOCUMENT_CLASS_PROBABILITY_FILTER_PARAMS: f64 = 0.8;
