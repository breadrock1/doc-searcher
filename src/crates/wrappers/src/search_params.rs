use crate::bucket::DEFAULT_BUCKET_NAME;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema, Clone)]
pub struct SearchParams {
    pub query: String,
    pub buckets: Option<String>,
    pub document_type: String,
    pub document_extension: String,
    pub document_size_to: i64,
    pub document_size_from: i64,
    pub created_date_to: String,
    pub created_date_from: String,
    pub result_size: i64,
    pub result_offset: i64,
    pub scroll_timelife: String,
}

impl SearchParams {
    pub fn get_scroll(&self) -> &str {
        self.scroll_timelife.as_str()
    }
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParamsBuilder::default()
            .query("*".to_string())
            .buckets(Some(DEFAULT_BUCKET_NAME.to_string()))
            .document_type(String::default())
            .document_extension(String::default())
            .created_date_to(String::default())
            .created_date_from(String::default())
            .document_size_to(0)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .scroll_timelife("30m".to_string())
            .build()
            .unwrap()
    }
}
