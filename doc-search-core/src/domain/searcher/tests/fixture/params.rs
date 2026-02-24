#![allow(dead_code)]

use rstest::fixture;

use crate::domain::searcher::models::{
    FilterParams, FilterParamsBuilder, FullTextSearchingParamsBuilder,
    HybridSearchingParamsBuilder, ResultOrder, ResultParams, ResultParamsBuilder,
    RetrieveIndexDocumentsParamsBuilder, SearchKindParams, SearchingParams,
    SemanticSearchingParamsBuilder,
};
use crate::domain::searcher::tests::fixture::*;

#[fixture]
pub fn build_retrieve_searching_params() -> SearchingParams {
    let indexes = vec!["test-index-1".into(), "test-index-2".into()];
    let result = build_result_searching_params();
    let filter = build_filter_searching_params();
    let params = RetrieveIndexDocumentsParamsBuilder::default()
        .path(Some(QUERY_FIELD_VALUE.to_string()))
        .build()
        .expect("build retrieve searching params failed");
    SearchingParams::new(
        indexes,
        SearchKindParams::Retrieve(params),
        result,
        Some(filter),
    )
}

#[fixture]
pub fn build_full_text_searching_params() -> SearchingParams {
    let indexes = vec!["test-index-1".into(), "test-index-2".into()];
    let result = build_result_searching_params();
    let filter = build_filter_searching_params();
    let params = FullTextSearchingParamsBuilder::default()
        .query(Some(QUERY_FIELD_VALUE.to_string()))
        .build()
        .expect("build full text searching params failed");
    SearchingParams::new(
        indexes,
        SearchKindParams::FullText(params),
        result,
        Some(filter),
    )
}

#[fixture]
pub fn build_semantic_searching_params() -> SearchingParams {
    let indexes = vec!["test-index-1".into(), "test-index-2".into()];
    let result = build_result_searching_params();
    let filter = build_filter_searching_params();
    let params = SemanticSearchingParamsBuilder::default()
        .query(QUERY_FIELD_VALUE.to_string())
        .model_id(Some(EMBEDDINGS_MODEL_ID.to_string()))
        .tokens(None)
        .knn_amount(100)
        .min_score(Some(0.6))
        .build()
        .expect("build semantic search params failed");
    SearchingParams::new(
        indexes,
        SearchKindParams::Semantic(params),
        result,
        Some(filter),
    )
}

#[fixture]
pub fn build_hybrid_searching_params() -> SearchingParams {
    let indexes = vec!["test-index-1".into(), "test-index-2".into()];
    let result = build_result_searching_params();
    let filter = build_filter_searching_params();
    let params = HybridSearchingParamsBuilder::default()
        .query(QUERY_FIELD_VALUE.to_string())
        .model_id(Some(EMBEDDINGS_MODEL_ID.to_string()))
        .knn_amount(100)
        .min_score(Some(0.6))
        .build()
        .expect("hybrid searching params build failed");
    SearchingParams::new(
        indexes,
        SearchKindParams::Hybrid(params),
        result,
        Some(filter),
    )
}

#[fixture]
pub fn build_filter_searching_params() -> FilterParams {
    FilterParamsBuilder::default()
        .doc_part_id(Some(1))
        .size_from(Some(0))
        .size_to(Some(4096))
        .created_from(Some(CURRENT_TIMESTAMP))
        .created_to(Some(CURRENT_TIMESTAMP))
        .modified_from(Some(CURRENT_TIMESTAMP))
        .modified_to(Some(CURRENT_TIMESTAMP))
        .pipeline_id(Some(PIPELINE_ID_FILTER_PARAMS))
        .source(Some(SOURCE_FILTER_PARAMS.to_string()))
        .semantic_source(Some(SEMANTIC_SOURCE_FILTER_PARAMS.to_string()))
        .distance(Some(DISTANCE_FILTER_PARAMS.to_string()))
        .location_coords(Some(LOCATION_COORDS_FILTER_PARAMS.to_vec()))
        .doc_class(Some(DOCUMENT_CLASS_FILTER_PARAMS.to_string()))
        .doc_class_probability(Some(DOCUMENT_CLASS_PROBABILITY_FILTER_PARAMS))
        .build()
        .expect("should be able to build filter searching")
}

#[fixture]
pub fn build_result_searching_params() -> ResultParams {
    ResultParamsBuilder::default()
        .size(10)
        .offset(0)
        .order(ResultOrder::ASC)
        .highlight_items(Some(10))
        .highlight_item_size(Some(10))
        .include_extra_fields(Some(true))
        .build()
        .expect("should be able to build result searching")
}
