use rstest::rstest;

use crate::domain::searcher::models::SearchingParams;
use crate::domain::searcher::tests::fixture::params::*;

#[rstest]
#[case(build_retrieve_searching_params(), "retrieve")]
#[case(build_full_text_searching_params(), "fulltext")]
#[case(build_semantic_searching_params(), "semantic")]
#[case(build_hybrid_searching_params(), "hybrid")]
fn test_build_searching_params(#[case] searching_params: SearchingParams, #[case] kind: &str) {
    assert_eq!(2, searching_params.get_indexes().len());
    assert_eq!(kind, searching_params.get_kind().to_string());
    assert_eq!(10, searching_params.get_result().size);
    assert!(searching_params.get_filter().is_some());

    let debug = format!("{searching_params:?}");
    assert!(debug.contains("indexes:"));
    assert!(debug.contains("kind:"));
    assert!(debug.contains("result:"));
    assert!(debug.contains("filter:"));
}

#[test]
fn test_search_kind_display_all_variants() {
    assert_eq!(
        "retrieve",
        build_retrieve_searching_params().get_kind().to_string()
    );
    assert_eq!(
        "fulltext",
        build_full_text_searching_params().get_kind().to_string()
    );
    assert_eq!(
        "semantic",
        build_semantic_searching_params().get_kind().to_string()
    );
    assert_eq!(
        "hybrid",
        build_hybrid_searching_params().get_kind().to_string()
    );
}

#[test]
fn test_semantic_params_debug() {
    let semantic = build_semantic_searching_params();
    let debug = format!("{:?}", semantic.get_kind());
    assert!(debug.contains("Semantic"));
}

#[test]
fn test_hybrid_params_debug() {
    let hybrid = build_hybrid_searching_params();
    let debug = format!("{:?}", hybrid.get_kind());
    assert!(debug.contains("Hybrid"));
}
