#![allow(dead_code)]
use rstest::fixture;

use crate::domain::searcher::models::{FilterParams, FilterParamsBuilder};
use crate::domain::searcher::models::{FullTextSearchingParams, FullTextSearchingParamsBuilder};
use crate::domain::searcher::models::{HybridSearchingParams, HybridSearchingParamsBuilder};
use crate::domain::searcher::models::{ResultOrder, ResultParams, ResultParamsBuilder};
use crate::domain::searcher::models::{
    RetrieveIndexDocumentsParams, RetrieveIndexDocumentsParamsBuilder,
};
use crate::domain::searcher::models::{SemanticSearchingParams, SemanticSearchingParamsBuilder};

pub const QUERY_PARAMETER: &str = "There is some query";
pub const DOCUMENT_PATH: &str = "./test-document.docx";
pub const SEARCH_MODEL_ID: &str = "p30o65gBnrvKdVIONWdC";
pub const CURRENT_TIMESTAMP: i64 = 1756498133;
pub const KNN_AMOUNT: u16 = 1024;
pub const MIN_SCORE: f32 = 0.6;
pub const SEARCH_TOKENS: &[f64] = &[-1.4354, 0.435435];

#[fixture]
pub fn build_simple_retrieve_params() -> RetrieveIndexDocumentsParams {
    RetrieveIndexDocumentsParamsBuilder::default()
        .path(None)
        .build()
        .expect("failed building retrieve params builder")
}

#[fixture]
pub fn build_with_path_retrieve_params() -> RetrieveIndexDocumentsParams {
    let path = DOCUMENT_PATH.to_string();
    RetrieveIndexDocumentsParamsBuilder::default()
        .path(Some(path))
        .build()
        .expect("failed building retrieve params builder")
}

#[fixture]
pub fn build_simple_fulltext_params() -> FullTextSearchingParams {
    FullTextSearchingParamsBuilder::default()
        .query(None)
        .build()
        .expect("failed building full text params builder")
}

#[fixture]
pub fn build_with_query_fulltext_params() -> FullTextSearchingParams {
    let query = QUERY_PARAMETER.to_string();
    FullTextSearchingParamsBuilder::default()
        .query(Some(query))
        .build()
        .expect("failed building full text params builder")
}

#[fixture]
pub fn build_simple_semantic_params() -> SemanticSearchingParams {
    let query = QUERY_PARAMETER.to_string();
    SemanticSearchingParamsBuilder::default()
        .query(query)
        .model_id(Some(SEARCH_MODEL_ID.to_string()))
        .tokens(None)
        .knn_amount(KNN_AMOUNT)
        .min_score(None)
        .build()
        .expect("failed building semantic params builder")
}

#[fixture]
pub fn build_semantic_params_with_tokens() -> SemanticSearchingParams {
    let query = QUERY_PARAMETER.to_string();
    SemanticSearchingParamsBuilder::default()
        .query(query)
        .model_id(Some(SEARCH_MODEL_ID.to_string()))
        .tokens(Some(SEARCH_TOKENS.to_vec()))
        .knn_amount(KNN_AMOUNT)
        .min_score(Some(MIN_SCORE))
        .build()
        .expect("failed building semantic params builder")
}

#[fixture]
pub fn build_full_semantic_params() -> SemanticSearchingParams {
    let query = QUERY_PARAMETER.to_string();
    SemanticSearchingParamsBuilder::default()
        .query(query)
        .tokens(None)
        .min_score(Some(MIN_SCORE))
        .knn_amount(KNN_AMOUNT)
        .model_id(Some(SEARCH_MODEL_ID.to_string()))
        .build()
        .expect("failed building semantic params builder")
}

#[fixture]
pub fn build_simple_hybrid_params() -> HybridSearchingParams {
    let query = QUERY_PARAMETER.to_string();
    HybridSearchingParamsBuilder::default()
        .query(query)
        .model_id(Some(SEARCH_MODEL_ID.to_string()))
        .min_score(Some(MIN_SCORE))
        .knn_amount(KNN_AMOUNT)
        .build()
        .expect("failed building hybrid params builder")
}

#[fixture]
pub fn build_full_hybrid_params() -> HybridSearchingParams {
    let query = QUERY_PARAMETER.to_string();
    HybridSearchingParamsBuilder::default()
        .query(query)
        .knn_amount(KNN_AMOUNT)
        .model_id(Some(SEARCH_MODEL_ID.to_string()))
        .min_score(Some(MIN_SCORE))
        .build()
        .expect("failed building hybrid params builder")
}

pub fn build_result_params() -> ResultParams {
    ResultParamsBuilder::default()
        .order(ResultOrder::DESC)
        .size(10)
        .offset(0)
        .include_extra_fields(Some(true))
        .highlight_items(Some(10))
        .highlight_item_size(Some(100))
        .build()
        .expect("failed building result params builder")
}

pub fn build_filter_params() -> FilterParams {
    FilterParamsBuilder::default()
        .doc_part_id(None)
        .size_from(Some(0))
        .size_to(Some(4096))
        .created_from(Some(CURRENT_TIMESTAMP))
        .created_to(Some(CURRENT_TIMESTAMP))
        .modified_from(Some(CURRENT_TIMESTAMP))
        .modified_to(Some(CURRENT_TIMESTAMP))
        .build()
        .expect("failed building filter params builder")
}
