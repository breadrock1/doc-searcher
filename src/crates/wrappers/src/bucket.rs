use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use std::fmt::Display;

pub const DEFAULT_BUCKET_NAME: &str = "common_bucket";

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct Bucket {
    #[schema(example = "yellow")]
    pub health: String,
    #[schema(example = "open")]
    pub status: String,
    #[schema(example = "test_bucket")]
    pub index: String,
    #[schema(example = "60qbF_yuTa2TXYd7soYb1A")]
    pub uuid: String,
    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<String>,
    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep: Option<String>,
    #[schema(example = "100")]
    #[serde(alias = "docs.count", skip_serializing_if = "Option::is_none")]
    pub docs_count: Option<String>,
    #[schema(example = "50")]
    #[serde(alias = "docs.deleted", skip_serializing_if = "Option::is_none")]
    pub docs_deleted: Option<String>,
    #[schema(example = "890.3kb")]
    #[serde(alias = "store.size", skip_serializing_if = "Option::is_none")]
    pub store_size: Option<String>,
    #[schema(example = "890.3kb")]
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
