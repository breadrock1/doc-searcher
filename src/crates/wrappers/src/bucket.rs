use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use std::fmt::Display;

pub const DEFAULT_BUCKET_NAME: &str = "common_bucket";

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct Bucket {
    pub health: String,
    pub status: String,
    pub index: String,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep: Option<String>,
    #[serde(alias = "docs.count", skip_serializing_if = "Option::is_none")]
    pub docs_count: Option<String>,
    #[serde(alias = "docs.deleted", skip_serializing_if = "Option::is_none")]
    pub docs_deleted: Option<String>,
    #[serde(alias = "store.size", skip_serializing_if = "Option::is_none")]
    pub store_size: Option<String>,
    #[serde(alias = "pri.store.size", skip_serializing_if = "Option::is_none")]
    pub pri_store_size: Option<String>,
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
        BucketForm::new(DEFAULT_BUCKET_NAME)
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
