use doc_search_core::domain::searcher::models::{
    FilterParams, FilterParamsBuilder, FullTextSearchingParamsBuilder,
    HybridSearchingParamsBuilder, ResultOrder, ResultParams, ResultParamsBuilder,
    RetrieveIndexDocumentsParamsBuilder, SearchKindParams, SearchingParams,
    SemanticSearchingParamsBuilder,
};
use rstest::fixture;

pub const CURRENT_TIMESTAMP_FROM: i64 = 1740731675;
pub const CURRENT_TIMESTAMP_TO: i64 = 1760731675;
pub const FILE_SIZE_TO: u32 = 1334314;
pub const TEST_INDEX_ID: &str = "test-folder";
pub const DOCUMENT_FILE_PATH: &str = "./test-document.docx";
pub const SEARCHING_QUERY: &str = "Intuition is very important to understanding a concept.";

pub fn build_indexes() -> Vec<String> {
    vec![TEST_INDEX_ID.to_string()]
}

pub fn build_result_params() -> ResultParams {
    ResultParamsBuilder::default()
        .order(ResultOrder::DESC)
        .size(10)
        .offset(0)
        .include_extra_fields(Some(true))
        .highlight_items(None)
        .highlight_item_size(None)
        .build()
        .expect("failed building result params builder")
}

pub fn build_filter_params() -> FilterParams {
    FilterParamsBuilder::default()
        .doc_part_id(None)
        .size_from(Some(0))
        .size_to(Some(FILE_SIZE_TO))
        .created_from(Some(CURRENT_TIMESTAMP_FROM))
        .created_to(Some(CURRENT_TIMESTAMP_TO))
        .modified_from(Some(CURRENT_TIMESTAMP_FROM))
        .modified_to(Some(CURRENT_TIMESTAMP_TO))
        .build()
        .expect("failed building filter params builder")
}

#[fixture]
pub fn build_simple_retrieve_params() -> SearchingParams {
    let path = DOCUMENT_FILE_PATH.to_string();
    let result = build_result_params();
    let filter = build_filter_params();
    let indexes = build_indexes();
    let params = RetrieveIndexDocumentsParamsBuilder::default()
        .path(Some(path))
        .build()
        .expect("failed building retrieve params builder");

    let kind = SearchKindParams::Retrieve(params);
    SearchingParams::new(indexes, kind, result, Some(filter))
}

#[fixture]
pub fn build_simple_fulltext_params() -> SearchingParams {
    let result = build_result_params();
    let filter = build_filter_params();
    let indexes = build_indexes();
    let params = FullTextSearchingParamsBuilder::default()
        .query(None)
        .build()
        .expect("failed building full text params builder");

    let kind = SearchKindParams::FullText(params);
    SearchingParams::new(indexes, kind, result, Some(filter))
}

#[fixture]
pub fn build_simple_semantic_params() -> SearchingParams {
    let query = SEARCHING_QUERY.to_string();
    let result = build_result_params();
    let filter = build_filter_params();
    let indexes = build_indexes();
    let params = SemanticSearchingParamsBuilder::default()
        .query(query)
        .model_id(None)
        .tokens(None)
        .min_score(None)
        .knn_amount(100)
        .build()
        .expect("failed building semantic params builder");

    let kind = SearchKindParams::Semantic(params);
    SearchingParams::new(indexes, kind, result, Some(filter))
}

#[fixture]
pub fn build_simple_hybrid_params() -> SearchingParams {
    let query = SEARCHING_QUERY.to_string();
    let result = build_result_params();
    let filter = build_filter_params();
    let indexes = build_indexes();
    let params = HybridSearchingParamsBuilder::default()
        .query(query)
        .model_id(None)
        .min_score(None)
        .knn_amount(100)
        .build()
        .expect("failed building hybrid params builder");

    let kind = SearchKindParams::Hybrid(params);
    SearchingParams::new(indexes, kind, result, Some(filter))
}
