use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::application::dto::Tokens;
#[allow(unused_imports)]
use serde_json::json;

pub trait QueryBuilder {
    fn build_query(&self, extra_field: Option<&str>) -> serde_json::Value;
}

#[derive(Getters, Serialize, Deserialize, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct FilterParams {
    #[schema(example = 0)]
    size_from: Option<u32>,
    #[schema(example = 1024)]
    size_to: Option<u32>,
    #[schema(example = 1750957115)]
    created_from: Option<i64>,
    #[schema(example = 1750957215)]
    created_to: Option<i64>,
    #[schema(example = 1750957115)]
    modified_from: Option<i64>,
    #[schema(example = 1750957215)]
    modified_to: Option<i64>,
}

#[derive(Clone, CopyGetters, Getters, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ResultParams {
    #[schema(example = "desc")]
    #[getset(get = "pub")]
    order: String,
    #[schema(example = 10)]
    #[getset(get_copy = "pub")]
    size: i64,
    #[schema(example = 0)]
    #[getset(get_copy = "pub")]
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
pub struct HybridSearchParams {
    #[schema(example = "Hello world")]
    #[getset(get = "pub")]
    query: String,
    #[schema(example = 5)]
    #[getset(get_copy = "pub")]
    knn_amount: Option<u16>,
    #[getset(get = "pub")]
    filter: Option<FilterParams>,
    #[getset(get = "pub")]
    result: ResultParams,
    #[schema(example = "test-folder-1,test-folder-2")]
    #[getset(get = "pub")]
    indexes: String,
    #[schema(example = "PRh30JcBW8Qg3Gf4I6Ku")]
    #[getset(get = "pub")]
    model_id: Option<String>,
}

#[derive(Getters, CopyGetters, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct SemanticSearchParams {
    #[schema(example = "Hello world")]
    #[getset(get = "pub")]
    query: String,
    #[schema(example = 5)]
    #[getset(get_copy = "pub")]
    knn_amount: Option<u16>,
    #[getset(get = "pub")]
    result: ResultParams,
    #[schema(example = "test-folder-1,test-folder-2")]
    #[getset(get = "pub")]
    indexes: String,
    #[schema(example = "PRh30JcBW8Qg3Gf4I6Ku")]
    #[getset(get = "pub")]
    model_id: Option<String>,
}

#[derive(Builder, Getters, CopyGetters, Serialize)]
pub struct SemanticSearchWithTokensParams {
    #[getset(get = "pub")]
    tokens: Vec<f64>,
    #[getset(get_copy = "pub")]
    knn_amount: Option<u16>,
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
            .result(params.result().to_owned())
            .indexes(params.indexes().to_owned())
            .tokens(tokens.tokens().to_owned())
            .build()
            .unwrap()
    }
}

#[derive(Builder, Getters, Deserialize, Serialize, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct PaginateParams {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    scroll_id: String,
    #[schema(example = "5m")]
    lifetime: String,
}

impl PaginateParams {
    pub fn builder() -> PaginateParamsBuilder {
        PaginateParamsBuilder::default()
    }
}
