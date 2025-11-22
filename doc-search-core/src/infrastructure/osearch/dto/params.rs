use derive_builder::Builder;
use gset::Getset;

use crate::domain::searcher::models::{FilterParams, ResultParams};

#[derive(Debug, Builder, Getset)]
pub struct RetrieveAllDocPartsQueryParams {
    #[getset(get, vis = "pub")]
    large_doc_id: String,
    #[getset(get_copy, vis = "pub")]
    with_sorting: bool,
    #[getset(get_copy, vis = "pub")]
    only_first_part: bool,
}

#[derive(Debug, Builder, Getset)]
pub struct RetrieveIndexDocsQueryParams {
    #[getset(get, vis = "pub")]
    path: Option<String>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
}

impl RetrieveIndexDocsQueryParams {
    pub fn get_excluded_params(&self) -> &[&str] {
        let include_extra_fields = self.result.include_extra_fields.unwrap_or_default();
        if include_extra_fields {
            return &["chunked_text", "embeddings"];
        };

        &["content", "chunked_text", "embeddings"]
    }
}

#[derive(Debug, Builder, Getset)]
pub struct FullTextQueryParams {
    #[getset(get, vis = "pub")]
    query: Option<String>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
}

impl FullTextQueryParams {
    pub fn get_excluded_params(&self) -> &[&str] {
        let include_extra_fields = self.result.include_extra_fields.unwrap_or_default();
        if include_extra_fields {
            return &["chunked_text", "embeddings"];
        };

        &["content", "chunked_text", "embeddings"]
    }
}

#[derive(Debug, Builder, Getset)]
pub struct SemanticQueryParams {
    #[getset(get, vis = "pub")]
    query: String,
    #[getset(get, vis = "pub")]
    model_id: String,
    #[getset(get_copy, vis = "pub")]
    knn_amount: u16,
    #[getset(get_copy, vis = "pub")]
    min_score: Option<f32>,
    #[getset(get, vis = "pub")]
    tokens: Option<Vec<f64>>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
}

impl SemanticQueryParams {
    pub fn get_excluded_params(&self) -> &[&str] {
        let include_extra_fields = self.result.include_extra_fields.unwrap_or_default();
        if include_extra_fields {
            return &["content"];
        };

        &["content", "chunked_text", "embeddings"]
    }
}

#[derive(Debug, Builder, Getset)]
pub struct HybridQueryParams {
    #[getset(get, vis = "pub")]
    query: String,
    #[getset(get, vis = "pub")]
    model_id: String,
    #[getset(get_copy, vis = "pub")]
    knn_amount: u16,
    #[getset(get_copy, vis = "pub")]
    min_score: Option<f32>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
}

impl HybridQueryParams {
    pub fn get_excluded_params(&self) -> &[&str] {
        let exclude_extra_fields = self.result.include_extra_fields.unwrap_or_default();
        if exclude_extra_fields {
            return &["chunked_text", "embeddings"];
        }

        &["content"]
    }
}
