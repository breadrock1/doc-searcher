use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
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

impl MustQuery {
    pub fn new(value: Value) -> Self {
        MustQuery { must: value }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BoolQuery {
    #[serde(alias = "bool")]
    bool: MustQuery,
}

impl BoolQuery {
    pub fn new(value: MustQuery) -> Self {
        BoolQuery { bool: value }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RangeQuery {
    range: Value,
}

impl RangeQuery {
    pub fn new(value: Value) -> Self {
        RangeQuery { range: value }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    bool: MustQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<BoolQuery>,
}

impl SearchQuery {
    pub fn new1(match_query: MustQuery) -> Self {
        SearchQuery {
            bool: match_query,
            filter: None,
        }
    }

    pub fn new2(match_query: MustQuery, filter_query: BoolQuery) -> Self {
        SearchQuery {
            bool: match_query,
            filter: Some(filter_query),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MainQueryNode {
    query: SearchQuery,
}

impl MainQueryNode {
    pub fn new(value: SearchQuery) -> Self {
        MainQueryNode { query: value }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchParameters {
    pub query: String,
    pub document_size_to: i64,
    pub document_size_from: i64,
    pub created_date_to: i64,
    pub created_date_from: i64,
    pub result_size: i64,
    pub result_offset: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Document {
    pub bucket_uuid: String,
    pub bucket_path: String,
    pub document_name: String,
    pub document_path: String,
    pub document_size: i32,
    pub document_type: String,
    pub document_extension: String,
    pub document_permissions: i32,
    pub document_md5_hash: String,
    pub document_ssdeep_hash: String,
    pub entity_keywords: Vec<String>,
    #[serde(
        serialize_with = "serialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_created: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_modified: Option<DateTime<Utc>>,
}

pub fn serialize_dt<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(dt) = dt {
        dt.format("%Y-%m-%dT%H:%M:%SZ")
            .to_string()
            .serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct StatusResult {
    result: u16,
}

impl StatusResult {
    pub fn new(status: u16) -> Self {
        StatusResult { result: status }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Cluster {
    ip: String,
    #[serde(alias = "heap.percent")]
    heap_percent: String,
    #[serde(alias = "ram.percent")]
    ram_percent: String,
    cpu: String,
    load_1m: String,
    load_5m: String,
    load_15m: String,
    #[serde(alias = "node.role")]
    node_role: String,
    master: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClusterForm {
    cluster_name: String,
}

impl ToString for ClusterForm {
    fn to_string(&self) -> String {
        let self_data = &self.cluster_name;
        self_data.clone()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Bucket {
    health: String,
    status: String,
    index: String,
    uuid: String,
    pri: String,
    rep: String,
    #[serde(alias = "docs.count")]
    docs_count: String,
    #[serde(alias = "docs.deleted")]
    docs_deleted: String,
    #[serde(alias = "store.size")]
    store_size: String,
    #[serde(alias = "pri.store.role")]
    pri_store_size: String,
}

#[derive(Deserialize, Serialize)]
pub struct BucketForm {
    bucket_name: String,
}

impl ToString for BucketForm {
    fn to_string(&self) -> String {
        let self_data = &self.bucket_name;
        self_data.clone()
    }
}
