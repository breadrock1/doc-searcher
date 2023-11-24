use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
#[serde(rename = "document_size")]
pub struct DocumentSizeQuery {
    gte: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<i64>,
}

impl DocumentSizeQuery {
    pub fn new(gte: i64, lte: i64) -> Self {
        let lte_value = match lte > 0 {
            true => Some(lte),
            false => None,
        };

        DocumentSizeQuery {
            gte,
            lte: lte_value,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct QueryString {
    query: String,
}

impl QueryString {
    pub fn new(value: String) -> Self {
        QueryString { query: value }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MultiMatchQuery {
    multi_match: QueryString,
}

impl MultiMatchQuery {
    pub fn new(value: QueryString) -> Self {
        MultiMatchQuery { multi_match: value }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MustQuery {
    must: Value,
}

#[derive(Serialize, Deserialize)]
pub struct BoolQuery {
    #[serde(alias = "bool")]
    bool: MustQuery,
}

#[derive(Serialize, Deserialize)]
pub struct RangeQuery {
    range: Value,
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    bool: MustQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<BoolQuery>,
}

#[derive(Serialize, Deserialize)]
pub struct MainQueryNode {
    query: SearchQuery,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchParams {
    pub query: String,
    pub document_type: String,
    pub document_path: String,
    pub document_extension: String,
    pub document_size_to: i64,
    pub document_size_from: i64,
    pub created_date_to: i64,
    pub created_date_from: i64,
    pub result_size: i64,
    pub result_offset: i64,
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParams {
            query: "*".to_string(),
            document_type: "*".to_string(),
            document_path: "*".to_string(),
            document_extension: "*".to_string(),
            document_size_to: 0,
            document_size_from: 0,
            created_date_to: 0,
            created_date_from: 0,
            result_size: 25,
            result_offset: 0,
        }
    }
}
