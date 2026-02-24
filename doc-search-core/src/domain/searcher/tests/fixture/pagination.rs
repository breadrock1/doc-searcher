use crate::domain::searcher::models::{
    RetrieveIndexDocumentsParamsBuilder, SearchKindParams, SearchingParams,
};
use crate::domain::searcher::tests::fixture::QUERY_FIELD_VALUE;
use crate::domain::searcher::tests::fixture::params::{
    build_filter_searching_params, build_result_searching_params,
};

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
