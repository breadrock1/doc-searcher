use derive_builder::Builder;
use gset::Getset;

#[derive(Clone, Debug, Builder, Getset)]
pub struct FilterParams {
    #[getset(get_copy, vis = "pub")]
    doc_part_id: Option<usize>,
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

#[derive(Clone, Debug, Builder, Getset)]
pub struct ResultParams {
    #[getset(get, vis = "pub")]
    order: String,
    #[getset(get_copy, vis = "pub")]
    size: i64,
    #[getset(get_copy, vis = "pub")]
    offset: i64,
    #[getset(get_copy, vis = "pub")]
    include_extra_fields: Option<bool>,
    #[getset(get_copy, vis = "pub")]
    highlight_items: Option<u32>,
    #[getset(get_copy, vis = "pub")]
    highlight_item_size: Option<u32>,
}

#[derive(Builder, Debug, Getset)]
pub struct RetrieveParams {
    #[getset(get, vis = "pub")]
    path: Option<String>,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
}

#[derive(Builder, Debug, Getset)]
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

#[derive(Builder, Debug, Getset)]
pub struct HybridSearchParams {
    #[getset(get, vis = "pub")]
    query: String,
    #[getset(get, vis = "pub")]
    indexes: String,
    #[getset(get, vis = "pub")]
    model_id: Option<String>,
    #[getset(get_copy, vis = "pub")]
    knn_amount: Option<u16>,
    #[getset(get_copy, vis = "pub")]
    min_score: Option<f32>,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
}

#[derive(Builder, Debug, Getset)]
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
    #[getset(get_copy, vis = "pub")]
    min_score: Option<f32>,
    #[getset(get, vis = "pub")]
    filter: Option<FilterParams>,
    #[getset(get, vis = "pub")]
    result: ResultParams,
}

#[derive(Debug, Builder, Getset)]
pub struct PaginateParams {
    #[getset(get, vis = "pub")]
    scroll_id: String,
    #[getset(get, vis = "pub")]
    lifetime: String,
}
