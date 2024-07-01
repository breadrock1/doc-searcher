use crate::forms::TestExample;
use crate::forms::documents::forms::DocumentType;
use crate::forms::folders::folder::DEFAULT_FOLDER_ID;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema, Clone)]
pub struct SearchParams {
    #[schema(example = "Hello world")]
    query: String,
    #[schema(example = "test-folder")]
    folder_ids: Option<String>,
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
    #[schema(example = 5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    knn_amount: Option<u16>,
    #[schema(example = 100)]
    #[serde(skip_serializing_if = "Option::is_none")]
    knn_candidates: Option<u32>,
    #[schema(example = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    show_all: Option<bool>
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
            self.created_date_to.as_str(),
        )
    }
    pub fn get_results_params(&self) -> (i64, i64) {
        (self.result_size, self.result_offset)
    }
    pub fn get_scroll(&self) -> &str {
        self.scroll_lifetime.as_str()
    }
    pub fn get_folders(&self, all_buckets: bool) -> String {
        match &self.folder_ids {
            None if all_buckets => "*".to_string(),
            None => DEFAULT_FOLDER_ID.to_string(),
            Some(data) => data.clone(),
        }
    }
    pub fn get_kkn_amount(&self) -> u16 {
        self.knn_amount.unwrap_or(5u16)
    }
    pub fn get_candidates(&self) -> u32 {
        self.knn_candidates.unwrap_or(100u32)
    }
    pub fn set_show_all(&mut self, flag: bool) {
        self.show_all = Some(flag);
    }
    pub fn get_show_all(&self) -> bool {
        self.show_all.unwrap_or(false)
    }
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParams::builder()
            .query("*".to_string())
            .folder_ids(Some(DEFAULT_FOLDER_ID.to_string()))
            .document_type(String::default())
            .document_extension(String::default())
            .created_date_to(String::default())
            .created_date_from(String::default())
            .document_size_to(0)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .scroll_lifetime("10m".to_string())
            .knn_amount(Some(5))
            .knn_candidates(Some(100))
            .show_all(Some(true))
            .build()
            .unwrap()
    }
}

impl TestExample<SearchParams> for SearchParams {
    fn test_example(query: Option<&str>) -> SearchParams {
        SearchParams::builder()
            .query(query.unwrap().to_string())
            .folder_ids(Some("test-folder".to_string()))
            .document_type("document".to_string())
            .document_extension("txt".to_string())
            .created_date_to("2024-04-26T11:14:55Z".to_string())
            .created_date_from("2024-04-02T13:51:32Z".to_string())
            .document_size_to(37000)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .scroll_lifetime("1m".to_string())
            .knn_amount(Some(5))
            .knn_candidates(Some(100))
            .show_all(Some(true))
            .build()
            .unwrap()
    }
}

#[derive(Deserialize, Default, IntoParams, ToSchema)]
pub struct SearchQuery {
    document_type: Option<DocumentType>,
}

impl SearchQuery {
    pub fn get_type(&self) -> DocumentType {
        self.document_type.clone().unwrap_or(DocumentType::Document)
    }
}
