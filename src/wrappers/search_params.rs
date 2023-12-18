use derive_builder::Builder;
use serde::Deserialize;

#[derive(Deserialize, Builder)]
pub struct SearchParams {
    pub query: String,
    pub document_type: String,
    pub document_path: String,
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
            .document_path(String::default())
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
