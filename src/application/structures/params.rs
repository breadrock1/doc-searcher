use derive_builder::Builder;
use gset::Getset;
use serde_derive::{Deserialize, Serialize};

const KNN_EF_SEARCHER: u32 = 100;
const KNN_DIMENSION: u32 = 384;
const TOKEN_LIMIT: u32 = 700;
const OVERLAP_RATE: f32 = 0.2;

#[derive(Builder, Clone, Debug, Getset, Serialize, Deserialize)]
pub struct CreateIndexParams {
    #[getset(set, vis = "pub")]
    #[getset(get, vis = "pub")]
    id: String,
    #[getset(get, vis = "pub")]
    name: String,
    #[getset(get, vis = "pub")]
    path: String,
    #[getset(get, vis = "pub")]
    knn: Option<KnnIndexParams>,
}

#[derive(Builder, Clone, Debug, Getset, Serialize, Deserialize)]
pub struct KnnIndexParams {
    #[getset(get_copy, vis = "pub")]
    knn_ef_searcher: u32,
    #[getset(get_copy, vis = "pub")]
    knn_dimension: u32,
    #[getset(get_copy, vis = "pub")]
    token_limit: u32,
    #[getset(get_copy, vis = "pub")]
    overlap_rate: f32,
}

impl Default for KnnIndexParams {
    fn default() -> Self {
        KnnIndexParams {
            knn_ef_searcher: KNN_EF_SEARCHER,
            knn_dimension: KNN_DIMENSION,
            token_limit: TOKEN_LIMIT,
            overlap_rate: OVERLAP_RATE,
        }
    }
}

#[derive(Builder, Clone, Getset, Serialize, Deserialize)]
pub struct FilterParams {
    #[getset(get_copy, vis = "pub")]
    size_from: Option<u32>,
    #[getset(get_copy, vis = "pub")]
    size_to: Option<u32>,
    #[getset(get_copy, vis = "pub")]
    created_from: Option<i64>,
    #[getset(get_copy, vis = "pub")]
    created_to: Option<i64>,
    #[getset(get_copy, vis = "pub")]
    modified_from: Option<i64>,
    #[getset(get_copy, vis = "pub")]
    modified_to: Option<i64>,
}

#[derive(Builder, Clone, Getset, Serialize, Deserialize)]
pub struct ResultParams {
    #[getset(get, vis = "pub")]
    order: String,
    #[getset(get_copy, vis = "pub")]
    size: i64,
    #[getset(get_copy, vis = "pub")]
    offset: i64,
    #[getset(get_copy, vis = "pub")]
    include_extra_fields: Option<bool>,
}

#[derive(Builder, Getset, Serialize, Deserialize)]
pub struct RetrieveDocumentParams {
    #[getset(get, vis = "pub")]
    path: Option<String>,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
}

#[derive(Builder, Getset, Serialize, Deserialize)]
pub struct FullTextSearchParams {
    #[getset(get, vis = "pub")]
    query: Option<String>,
    #[getset(get, vis = "pub")]
    indexes: String,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
}

#[derive(Builder, Getset, Serialize, Deserialize)]
pub struct HybridSearchParams {
    #[getset(get, vis = "pub")]
    query: String,
    #[getset(get, vis = "pub")]
    indexes: String,
    #[getset(get, vis = "pub")]
    model_id: Option<String>,
    #[getset(get_copy, vis = "pub")]
    knn_amount: Option<u16>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
}

#[derive(Builder, Getset, Serialize, Deserialize)]
pub struct SemanticSearchParams {
    #[getset(get, vis = "pub")]
    query: String,
    #[getset(get, vis = "pub")]
    indexes: String,
    #[getset(get, vis = "pub")]
    model_id: Option<String>,
    #[getset(get, vis = "pub")]
    tokens: Option<Vec<f64>>,
    #[getset(get_copy, vis = "pub")]
    knn_amount: Option<u16>,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
}

#[derive(Builder, Getset, Deserialize, Serialize)]
pub struct PaginateParams {
    #[getset(get, vis = "pub")]
    scroll_id: String,
    #[getset(get, vis = "pub")]
    lifetime: String,
}
