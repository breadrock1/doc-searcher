use crate::bucket::DEFAULT_FOLDER_NAME;
use crate::TestExample;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema, Clone)]
pub struct SearchParams {
    #[schema(example = "Hello world")]
    pub query: String,

    #[schema(example = "test_folder")]
    pub buckets: Option<String>,

    #[schema(example = "document")]
    pub document_type: String,

    #[schema(example = "txt")]
    pub document_extension: String,

    #[schema(example = 0)]
    pub document_size_to: i64,

    #[schema(example = 0)]
    pub document_size_from: i64,

    #[schema(example = "2024-04-26T11:14:55Z")]
    pub created_date_to: String,

    #[schema(example = "2024-04-02T13:51:32Z")]
    pub created_date_from: String,

    #[schema(example = 10)]
    pub result_size: i64,

    #[schema(example = 0)]
    pub result_offset: i64,

    #[schema(example = "1m")]
    pub scroll_timelife: String,
}

impl SearchParams {
    pub fn builder() -> SearchParamsBuilder {
        SearchParamsBuilder::default()
    }

    pub fn get_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn get_scroll(&self) -> &str {
        self.scroll_timelife.as_str()
    }
}

impl TestExample<SearchParams> for SearchParams {
    fn test_example(query: Option<&str>) -> SearchParams {
        SearchParams::builder()
            .query(query.unwrap().to_string())
            .buckets(Some("test_folder".to_string()))
            .document_type("document".to_string())
            .document_extension("txt".to_string())
            .created_date_to("2024-04-26T11:14:55Z".to_string())
            .created_date_from("2024-04-02T13:51:32Z".to_string())
            .document_size_to(37000)
            .document_size_from(32000)
            .result_size(25)
            .result_offset(0)
            .scroll_timelife("1m".to_string())
            .build()
            .unwrap()
    }
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParams::builder()
            .query("*".to_string())
            .buckets(Some(DEFAULT_FOLDER_NAME.to_string()))
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
