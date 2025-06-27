use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::application::dto::Tokens;
#[allow(unused_imports)]
use serde_json::json;

pub trait QueryBuilder {
    fn build_query(&self) -> serde_json::Value;
}

#[derive(Getters, Serialize, Deserialize, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct FilterParams {
    #[schema(example = json!(vec![0, 1024]))]
    size_range: Option<[u32; 2]>,
    #[schema(example = json!(vec![1750957115, 1750957215]))]
    created_range: Option<[i64; 2]>,
    #[schema(example = json!(vec![1750957115, 1750957215]))]
    modified_range: Option<[i64; 2]>,
}

#[derive(Clone, CopyGetters, Serialize, Deserialize, IntoParams, ToSchema)]
#[getset(get_copy = "pub")]
pub struct ResultParams {
    #[schema(example = 10)]
    size: i64,
    #[schema(example = 0)]
    offset: i64,
}

#[derive(Getters, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct RetrieveDocumentParams {
    #[schema(example = "./test-document.docx")]
    #[getset(get = "pub")]
    path: Option<String>,
    #[getset(get = "pub")]
    filter: Option<FilterParams>,
    #[getset(get = "pub")]
    result: ResultParams,
    #[schema(example = "test-folder-1,test-folder-2")]
    #[getset(get = "pub")]
    indexes: String,
}

#[derive(Getters, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct FullTextSearchParams {
    #[schema(example = "Hello world")]
    #[getset(get = "pub")]
    query: Option<String>,
    #[getset(get = "pub")]
    filter: Option<FilterParams>,
    #[getset(get = "pub")]
    result: ResultParams,
    #[schema(example = "test-folder-1,test-folder-2")]
    #[getset(get = "pub")]
    indexes: String,
}

#[derive(Getters, CopyGetters, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct SemanticSearchParams {
    #[schema(example = "Hello world")]
    #[getset(get = "pub")]
    query: String,
    #[schema(example = 5)]
    #[getset(get_copy = "pub")]
    knn_amount: Option<u16>,
    #[schema(example = 3)]
    #[getset(get_copy = "pub")]
    knn_candidates: Option<u32>,
    #[getset(get = "pub")]
    result: ResultParams,
    #[schema(example = "test-folder-1,test-folder-2")]
    #[getset(get = "pub")]
    indexes: String,
}

#[derive(Builder, Getters, CopyGetters, Serialize)]
pub struct SemanticSearchWithTokensParams {
    #[getset(get = "pub")]
    tokens: Vec<f64>,
    #[getset(get_copy = "pub")]
    knn_amount: Option<u16>,
    #[getset(get_copy = "pub")]
    knn_candidates: Option<u32>,
    #[getset(get = "pub")]
    result: ResultParams,
    #[getset(get = "pub")]
    indexes: String,
}

impl SemanticSearchWithTokensParams {
    pub fn build_from_semantic_params(
        params: &SemanticSearchParams,
        tokens: Tokens,
    ) -> SemanticSearchWithTokensParams {
        SemanticSearchWithTokensParamsBuilder::default()
            .knn_amount(params.knn_amount())
            .knn_candidates(params.knn_candidates())
            .result(params.result().to_owned())
            .indexes(params.indexes().to_owned())
            .tokens(tokens.tokens().to_owned())
            .build()
            .unwrap()
    }
}

#[derive(Getters, Deserialize, Serialize, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct PaginateParams {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    scroll_id: String,
    #[schema(example = "5m")]
    lifetime: String,
}
