use crate::storage::models::DEFAULT_FOLDER_ID;

use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Clone, Deserialize, Serialize, IntoParams, ToSchema, Getters)]
pub struct SearchParams {
    #[getset(get = "pub")]
    #[schema(example = "Hello world")]
    query: String,
    #[schema(example = "test-folder")]
    folder_ids: Option<String>,
    #[getset(get = "pub")]
    #[schema(example = "document")]
    document_type: String,
    #[getset(get = "pub")]
    #[schema(example = "txt")]
    document_extension: String,
    #[getset(get_copy = "pub")]
    #[schema(example = 0)]
    document_size_to: i64,
    #[getset(get_copy = "pub")]
    #[schema(example = 0)]
    document_size_from: i64,
    #[getset(get = "pub")]
    #[schema(example = "2024-04-26T11:14:55Z")]
    created_date_to: String,
    #[getset(get = "pub")]
    #[schema(example = "2024-04-02T13:51:32Z")]
    created_date_from: String,
    #[getset(get_copy = "pub")]
    #[schema(example = 10)]
    result_size: i64,
    #[getset(get_copy = "pub")]
    #[schema(example = 0)]
    result_offset: i64,
    #[getset(get = "pub")]
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
    show_all: Option<bool>,
}

impl SearchParams {
    pub fn builder() -> SearchParamsBuilder {
        SearchParamsBuilder::default()
    }

    pub fn set_query(&mut self, query: &str) {
        self.query = query.to_string();
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

    pub fn is_show_all(&self) -> bool {
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

#[derive(Serialize, Builder, ToSchema)]
pub struct Paginated<D> {
    #[schema(value_type = Paginated<Vec<Document>>)]
    founded: D,
    #[schema(example = "10m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}

impl<D> Paginated<D> {
    pub fn new(founded: D) -> Self {
        Paginated {
            founded,
            scroll_id: None,
        }
    }

    pub fn new_with_id(founded: D, id: String) -> Self {
        Paginated {
            founded,
            scroll_id: Some(id),
        }
    }

    pub fn new_with_opt_id(founded: D, scroll_id: Option<String>) -> Self {
        Paginated { founded, scroll_id }
    }

    pub fn get_founded(&self) -> &D {
        &self.founded
    }

    pub fn get_founded_mut(&mut self) -> &mut D {
        &mut self.founded
    }

    pub fn get_scroll_id(&self) -> Option<&String> {
        self.scroll_id.as_ref()
    }
}
