use crate::forms::folder::DEFAULT_FOLDER_ID;
use crate::forms::TestExample;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema, Clone)]
pub struct SearchParams {
    #[schema(example = "Hello world")]
    query: String,
    #[schema(example = "test_folder")]
    folders: Option<String>,
    #[schema(example = "document")]
    document_type: String,
    #[schema(example = "txt")]
    document_extension: String,
    #[schema(example = 0)]
    document_size_to: i64,
    #[schema(example = 0)]
    document_size_from: i64,
    #[schema(example = "2024-04-26T11:14:55Z")]
    created_date_to: String,
    #[schema(example = "2024-04-02T13:51:32Z")]
    created_date_from: String,
    #[schema(example = 10)]
    result_size: i64,
    #[schema(example = 0)]
    result_offset: i64,
    #[schema(example = "1m")]
    scroll_lifetime: String,
}

impl SearchParams {
    pub fn builder() -> SearchParamsBuilder {
        SearchParamsBuilder::default()
    }

    pub fn get_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn set_query(&mut self, query: &str) {
        self.query = query.to_string();
    }

    pub fn get_type(&self) -> &str {
        self.document_type.as_str()
    }

    pub fn get_extension(&self) -> &str {
        self.document_extension.as_str()
    }

    pub fn get_doc_size(&self) -> (i64, i64) {
        (self.document_size_from, self.document_size_to)
    }

    pub fn get_doc_dates(&self) -> (&str, &str) {
        (
            self.created_date_from.as_str(),
            self.created_date_from.as_str(),
        )
    }

    pub fn get_results_params(&self) -> (i64, i64) {
        (self.result_size, self.result_offset)
    }

    pub fn get_scroll(&self) -> &str {
        self.scroll_lifetime.as_str()
    }

    pub fn get_folders(&self, all_buckets: bool) -> String {
        match &self.folders {
            None if all_buckets => "*".to_string(),
            None => DEFAULT_FOLDER_ID.to_string(),
            Some(data) => data.clone(),
        }
    }
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParams::builder()
            .query("*".to_string())
            .folders(Some(DEFAULT_FOLDER_ID.to_string()))
            .document_type(String::default())
            .document_extension(String::default())
            .created_date_to(String::default())
            .created_date_from(String::default())
            .document_size_to(0)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .scroll_lifetime("10m".to_string())
            .build()
            .unwrap()
    }
}

impl TestExample<SearchParams> for SearchParams {
    fn test_example(query: Option<&str>) -> SearchParams {
        SearchParams::builder()
            .query(query.unwrap().to_string())
            .folders(Some("test_folder".to_string()))
            .document_type("document".to_string())
            .document_extension("txt".to_string())
            .created_date_to("2024-04-26T11:14:55Z".to_string())
            .created_date_from("2024-04-02T13:51:32Z".to_string())
            .document_size_to(37000)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .scroll_lifetime("1m".to_string())
            .build()
            .unwrap()
    }
}
