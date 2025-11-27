use rstest::rstest;

use crate::domain::searcher::models::SearchingParams;
use crate::domain::searcher::tests::fixture::params::*;

#[rstest]
#[case(build_retrieve_searching_params())]
#[case(build_full_text_searching_params())]
#[case(build_semantic_searching_params())]
#[case(build_hybrid_searching_params())]
fn test_build_searching_params(#[case] searching_params: SearchingParams) -> anyhow::Result<()> {
    let result = searching_params;
    assert_eq!(2, result.get_indexes().len());
    Ok(())
}
