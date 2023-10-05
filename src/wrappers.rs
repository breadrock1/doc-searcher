use std::os::linux::raw::stat;
use serde::{Deserialize, Serialize};

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
