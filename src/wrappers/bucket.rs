use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Builder, Default, Clone, ToSchema)]
pub struct Bucket {
    pub health: String,
    pub status: String,
    pub index: String,
    pub uuid: String,
    #[serde(alias = "docs.count")]
    pub docs_count: String,
    #[serde(alias = "docs.deleted")]
    pub docs_deleted: String,
    #[serde(alias = "store.size")]
    pub store_size: String,
    #[serde(alias = "pri.store.size")]
    pub pri_store_size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct BucketForm {
    bucket_name: String,
}

impl Display for BucketForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_data = &self.bucket_name;
        write!(f, "{}", self_data.clone())
    }
}

impl Default for BucketForm {
    fn default() -> Self {
        BucketForm {
            bucket_name: "common_bucket".to_string(),
        }
    }
}

impl BucketForm {
    pub fn new(bucket_name: &str) -> Self {
        BucketForm {
            bucket_name: bucket_name.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        self.bucket_name.as_str()
    }
}
