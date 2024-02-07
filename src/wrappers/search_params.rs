use cacher::values::{CacherSearchParams, CacherSearchParamsBuilder};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct SearchParams {
    pub query: String,
    pub document_type: String,
    pub document_extension: String,
    pub document_size_to: i64,
    pub document_size_from: i64,
    pub created_date_to: String,
    pub created_date_from: String,
    pub result_size: i64,
    pub result_offset: i64,
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParamsBuilder::default()
            .query("*".to_string())
            .document_type(String::default())
            .document_extension(String::default())
            .created_date_to(String::default())
            .created_date_from(String::default())
            .document_size_to(0)
            .document_size_from(0)
            .result_size(25)
            .result_offset(0)
            .build()
            .unwrap()
    }
}

impl From<&SearchParams> for CacherSearchParams {
    fn from(value: &SearchParams) -> Self {
        CacherSearchParamsBuilder::default()
            .query(value.query.to_owned())
            .document_type(value.document_type.to_owned())
            .document_extension(value.document_extension.to_owned())
            .created_date_to(value.created_date_to.to_owned())
            .created_date_from(value.created_date_from.to_owned())
            .document_size_to(value.document_size_to)
            .document_size_from(value.document_size_from)
            .result_size(value.result_size)
            .result_offset(value.result_offset)
            .build()
            .unwrap()
    }
}
