use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};

const KNN_EF_SEARCHER: u32 = 100;
const KNN_DIMENSION: u32 = 384;
const TOKEN_LIMIT: u32 = 50;
const OVERLAP_RATE: f32 = 0.2;

#[derive(Builder, Clone, Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct CreateIndexParams {
    id: String,
    name: String,
    path: String,
    knn: Option<KnnIndexParams>,
}

#[derive(Builder, Clone, CopyGetters, Serialize, Deserialize)]
#[getset(get_copy = "pub")]
pub struct KnnIndexParams {
    knn_ef_searcher: u32,
    knn_dimension: u32,
    token_limit: u32,
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

#[derive(Builder, Clone, CopyGetters, Serialize, Deserialize)]
#[getset(get_copy = "pub")]
pub struct FilterParams {
    size_from: Option<u32>,
    size_to: Option<u32>,
    created_from: Option<i64>,
    created_to: Option<i64>,
    modified_from: Option<i64>,
    modified_to: Option<i64>,
}

#[derive(Builder, Clone, CopyGetters, Getters, Serialize, Deserialize)]
pub struct ResultParams {
    #[getset(get = "pub")]
    order: String,
    #[getset(get_copy = "pub")]
    size: i64,
    #[getset(get_copy = "pub")]
    offset: i64,
    #[getset(get_copy = "pub")]
    include_extra_fields: Option<bool>,
}

#[derive(Builder, Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct RetrieveDocumentParams {
    path: Option<String>,
    filter: Option<FilterParams>,
    result: ResultParams,
}

#[derive(Builder, Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct FullTextSearchParams {
    query: Option<String>,
    filter: Option<FilterParams>,
    result: ResultParams,
    indexes: String,
}

#[derive(Builder, Getters, CopyGetters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct HybridSearchParams {
    query: String,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    knn_amount: Option<u16>,
    result: ResultParams,
    indexes: String,
    model_id: Option<String>,
}

#[derive(Builder, Getters, CopyGetters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct SemanticSearchParams {
    query: String,
    tokens: Option<Vec<f64>>,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    knn_amount: Option<u16>,
    filter: Option<FilterParams>,
    result: ResultParams,
    indexes: String,
    model_id: Option<String>,
}

#[derive(Builder, Getters, Deserialize, Serialize)]
#[getset(get = "pub")]
pub struct PaginateParams {
    scroll_id: String,
    lifetime: String,
}
