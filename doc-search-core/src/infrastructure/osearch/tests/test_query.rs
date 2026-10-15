use anyhow::Context;
use rstest::*;
use serde_json::{Value, json};

use crate::application::tests::fixture::search_params::*;
use crate::domain::searcher::models::{
    FullTextSearchingParams, FullTextSearchingParamsBuilder, HybridSearchingParams,
    HybridSearchingParamsBuilder, ResultOrder, ResultParams, ResultParamsBuilder,
    RetrieveIndexDocumentsParams, SearchKindParams, SearchingParams, SemanticSearchingParams,
    SemanticSearchingParamsBuilder,
};
use crate::domain::searcher::tests::fixture::params::{
    build_filter_searching_params, build_full_text_searching_params, build_hybrid_searching_params,
    build_retrieve_searching_params, build_semantic_searching_params,
};
use crate::infrastructure::osearch::dto::{
    FullTextQueryParamsBuilder, HybridQueryParamsBuilder, RetrieveAllDocPartsQueryParamsBuilder,
    RetrieveIndexDocsQueryParamsBuilder, SemanticQueryParamsBuilder,
};
use crate::infrastructure::osearch::query::{QueryBuildHelper, build_search_query};
use crate::infrastructure::osearch::tests::fixture::DOCUMENT_ID;

const RETRIEVE_FULL_PARAMS: &[u8] = include_bytes!("resources/retrieve-full-query.json");
const RETRIEVE_SIMPLE_PARAMS: &[u8] = include_bytes!("resources/retrieve-simple-query.json");
const FULLTEXT_FULL_PARAMS: &[u8] = include_bytes!("resources/fulltext-full-query.json");
const FULLTEXT_SIMPLE_PARAMS: &[u8] = include_bytes!("resources/fulltext-simple-query.json");
const SEMANTIC_FULL_PARAMS: &[u8] = include_bytes!("resources/semantic-full-query.json");
const SEMANTIC_SIMPLE_PARAMS: &[u8] = include_bytes!("resources/semantic-simple-query.json");
const SEMANTIC_TOKENS_PARAMS: &[u8] = include_bytes!("resources/semantic-tokens-query.json");
const HYBRID_FULL_PARAMS: &[u8] = include_bytes!("resources/hybrid-full-query.json");
const HYBRID_SIMPLE_PARAMS: &[u8] = include_bytes!("resources/hybrid-simple-query.json");
const PREFIX_FILTER: &str = "user-123/doc.txt";

#[rstest]
fn test_build_simplest_retrieve_params_query(
    #[from(build_simple_retrieve_params)] params: RetrieveIndexDocumentsParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = RetrieveIndexDocsQueryParamsBuilder::default()
        .path(params.path.clone())
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build retrieve query params")?;

    let query = query_params.build_query();

    let mut comparable_query = serde_json::from_slice::<Value>(RETRIEVE_SIMPLE_PARAMS)?;
    comparable_query["query"]["bool"]["filter"] = json!([]);
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_retrieve_params_query_from_with_path(
    #[from(build_with_path_retrieve_params)] params: RetrieveIndexDocumentsParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = RetrieveIndexDocsQueryParamsBuilder::default()
        .path(params.path.clone())
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build retrieve query params")?;

    let query = query_params.build_query();
    let mut comparable_query = serde_json::from_slice::<Value>(RETRIEVE_FULL_PARAMS)?;
    comparable_query["query"]["bool"]["filter"] = json!([]);
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_full_retrieve_params_query(
    #[from(build_with_path_retrieve_params)] params: RetrieveIndexDocumentsParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let filter = build_filter_searching_params();
    let query_params = RetrieveIndexDocsQueryParamsBuilder::default()
        .path(params.path.clone())
        .result(result.to_owned())
        .filter(Some(filter))
        .build()
        .context("failed to build retrieve query params")?;

    let query = query_params.build_query();
    let comparable_query = serde_json::from_slice::<Value>(RETRIEVE_FULL_PARAMS)?;
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_simple_fulltext_params_query(
    #[from(build_simple_fulltext_params)] params: FullTextSearchingParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = FullTextQueryParamsBuilder::default()
        .query(params.query.clone())
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build fulltext query params")?;

    let query = query_params.build_query();
    let mut comparable_query = serde_json::from_slice::<Value>(FULLTEXT_SIMPLE_PARAMS)?;
    comparable_query["query"]["bool"]["filter"] = json!([]);
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_full_fulltext_params_query(
    #[from(build_with_query_fulltext_params)] params: FullTextSearchingParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let filter = build_filter_searching_params();
    let query_params = FullTextQueryParamsBuilder::default()
        .query(params.query.clone())
        .result(result.to_owned())
        .filter(Some(filter))
        .build()
        .context("failed to build fulltext query params")?;

    let query = query_params.build_query();
    let comparable_query = serde_json::from_slice::<Value>(FULLTEXT_FULL_PARAMS)?;
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_simple_semantic_params_query(
    #[from(build_simple_semantic_params)] params: SemanticSearchingParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = SemanticQueryParamsBuilder::default()
        .query(params.query.clone())
        .tokens(Some(SEARCH_TOKENS.to_vec()))
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build semantic query params")?;

    let query = query_params.build_query();
    let mut comparable_query = serde_json::from_slice::<Value>(SEMANTIC_SIMPLE_PARAMS)?;
    comparable_query["query"]["bool"]["filter"] = json!([]);
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_semantic_params_query_with_tokens(
    #[from(build_semantic_params_with_tokens)] params: SemanticSearchingParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = SemanticQueryParamsBuilder::default()
        .query(params.query.clone())
        .tokens(Some(SEARCH_TOKENS.to_vec()))
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build semantic params query")?;

    let query = query_params.build_query();
    let comparable_query = serde_json::from_slice::<Value>(SEMANTIC_TOKENS_PARAMS)?;
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_full_semantic_params_query(
    #[from(build_full_semantic_params)] params: SemanticSearchingParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let filter = build_filter_searching_params();
    let query_params = SemanticQueryParamsBuilder::default()
        .query(params.query.clone())
        .tokens(Some(SEARCH_TOKENS.to_vec()))
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .result(result.to_owned())
        .filter(Some(filter))
        .build()
        .context("failed to build semantic query params")?;

    let query = query_params.build_query();
    let comparable_query = serde_json::from_slice::<Value>(SEMANTIC_FULL_PARAMS)?;
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_simple_hybrid_params_query(
    #[from(build_simple_hybrid_params)] params: HybridSearchingParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = HybridQueryParamsBuilder::default()
        .query(params.query.clone())
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .tokens(Some(SEARCH_TOKENS.to_vec()))
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build hybrid query params")?;

    let query = query_params.build_query();
    let comparable_query = serde_json::from_slice::<Value>(HYBRID_SIMPLE_PARAMS)?;
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
fn test_build_full_hybrid_params_query(
    #[from(build_full_hybrid_params)] params: HybridSearchingParams,
) -> anyhow::Result<()> {
    let result = build_result_params();
    let filter = build_filter_searching_params();
    let query_params = HybridQueryParamsBuilder::default()
        .query(params.query.clone())
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .tokens(Some(SEARCH_TOKENS.to_vec()))
        .result(result.to_owned())
        .filter(Some(filter))
        .build()
        .context("failed to build hybrid query params")?;

    let query = query_params.build_query();
    let comparable_query = serde_json::from_slice::<Value>(HYBRID_FULL_PARAMS)?;
    assert_eq!(query, comparable_query);

    Ok(())
}

#[rstest]
#[case(build_retrieve_searching_params())]
#[case(build_full_text_searching_params())]
#[case(build_semantic_searching_params())]
#[case(build_hybrid_searching_params())]
fn test_build_search_query_dispatcher(#[case] params: SearchingParams) -> anyhow::Result<()> {
    let query = build_search_query(params)?;
    assert!(query.is_object());
    Ok(())
}

#[test]
fn test_build_retrieve_all_doc_parts_query() -> anyhow::Result<()> {
    // only_first_part = true, with_sorting = true.
    let params = RetrieveAllDocPartsQueryParamsBuilder::default()
        .large_doc_id(DOCUMENT_ID.to_string())
        .with_sorting(true)
        .only_first_part(true)
        .build()
        .context("failed to build retrieve all doc parts query params")?;

    let query = params.build_query();
    assert!(
        query["query"]["bool"]["must"]
            .as_array()
            .is_some_and(|must| must.len() == 2)
    );
    assert_eq!(json!("ASC"), query["sort"]["doc_part_id"]["order"]);

    // only_first_part = false, with_sorting = false.
    let params = RetrieveAllDocPartsQueryParamsBuilder::default()
        .large_doc_id(DOCUMENT_ID.to_string())
        .with_sorting(false)
        .only_first_part(false)
        .build()
        .context("failed to build retrieve all doc parts query params")?;
    let query = params.build_query();
    assert!(
        query["query"]["bool"]["must"]
            .as_array()
            .is_some_and(|must| must.len() == 1)
    );
    assert!(query.get("sort").is_none());

    Ok(())
}

fn result_without_extra_fields() -> ResultParams {
    ResultParamsBuilder::default()
        .size(10)
        .offset(0)
        .order(ResultOrder::DESC)
        .highlight_items(Some(10))
        .highlight_item_size(Some(10))
        .include_extra_fields(Some(false))
        .build()
        .expect("failed to build result params")
}

fn assert_default_excluded(expected: &[&str], actual: &[&str]) {
    assert_eq!(expected, actual);
}

#[test]
fn test_get_excluded_params_default_branches() -> anyhow::Result<()> {
    let result = result_without_extra_fields();
    let retrieve = RetrieveIndexDocsQueryParamsBuilder::default()
        .path(None)
        .result(result.clone())
        .filter(None)
        .build()
        .context("retrieve")?;
    assert_default_excluded(
        &["content", "chunked_text", "embeddings"],
        retrieve.get_excluded_params(),
    );

    let fulltext = FullTextQueryParamsBuilder::default()
        .query(Some("query".to_string()))
        .result(result.clone())
        .filter(None)
        .build()
        .context("fulltext")?;
    assert_default_excluded(
        &["content", "chunked_text", "embeddings"],
        fulltext.get_excluded_params(),
    );

    let semantic = SemanticQueryParamsBuilder::default()
        .query("query".to_string())
        .knn_amount(10)
        .min_score(None)
        .tokens(None)
        .result(result.clone())
        .filter(None)
        .build()
        .context("semantic")?;
    assert_default_excluded(
        &["content", "chunked_text", "embeddings"],
        semantic.get_excluded_params(),
    );

    let hybrid = HybridQueryParamsBuilder::default()
        .query("query".to_string())
        .knn_amount(10)
        .min_score(None)
        .result(result)
        .filter(None)
        .build()
        .context("hybrid")?;
    assert_default_excluded(&["content"], hybrid.get_excluded_params());

    Ok(())
}

#[rstest]
fn test_build_semantic_query_without_tokens_uses_match_none() -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = SemanticQueryParamsBuilder::default()
        .query("query".to_string())
        .knn_amount(10)
        .min_score(None)
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build semantic query params")?;

    let query = query_params.build_query();
    let knn = &query["query"]["bool"]["must"][0]["nested"]["query"];
    assert_eq!(knn, &json!({ "match_none": {} }));
    Ok(())
}

#[rstest]
fn test_build_hybrid_query_without_tokens_uses_match_none() -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = HybridQueryParamsBuilder::default()
        .query("query".to_string())
        .knn_amount(10)
        .min_score(None)
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build hybrid query params")?;

    let query = query_params.build_query();
    let semantic = &query["query"]["hybrid"]["queries"][0];
    assert_eq!(semantic, &json!({ "match_none": {} }));
    Ok(())
}

#[rstest]
fn test_build_fulltext_query_with_prefix() -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = FullTextQueryParamsBuilder::default()
        .query(Some("query".to_string()))
        .prefix(Some(PREFIX_FILTER.to_string()))
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build fulltext query params")?;

    let query = query_params.build_query();
    let must = query["query"]["bool"]["must"]
        .as_array()
        .expect("must must be an array");
    assert!(must.contains(&json!({ "match": { "file_path": PREFIX_FILTER } })));
    Ok(())
}

#[rstest]
fn test_build_semantic_query_with_prefix() -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = SemanticQueryParamsBuilder::default()
        .query("query".to_string())
        .knn_amount(10)
        .min_score(None)
        .prefix(Some(PREFIX_FILTER.to_string()))
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build semantic query params")?;

    let query = query_params.build_query();
    let filter = query["query"]["bool"]["filter"]
        .as_array()
        .expect("filter must be an array");
    assert!(filter.contains(&json!({ "match": { "file_path": PREFIX_FILTER } })));
    Ok(())
}

#[rstest]
fn test_build_hybrid_query_with_prefix() -> anyhow::Result<()> {
    let result = build_result_params();
    let query_params = HybridQueryParamsBuilder::default()
        .query("query".to_string())
        .knn_amount(10)
        .min_score(None)
        .prefix(Some(PREFIX_FILTER.to_string()))
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build hybrid query params")?;

    let query = query_params.build_query();
    let filter = query["query"]["hybrid"]["queries"][1]["bool"]["filter"]
        .as_array()
        .expect("hybrid filter must be an array");
    assert!(filter.contains(&json!({ "match": { "file_path": PREFIX_FILTER } })));
    Ok(())
}

fn build_fulltext_searching_params_with_prefix() -> SearchingParams {
    SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::FullText(
            FullTextSearchingParamsBuilder::default()
                .query(Some("query".to_string()))
                .prefix(Some(PREFIX_FILTER.to_string()))
                .build()
                .expect("build fulltext params failed"),
        ),
        build_result_params(),
        None,
    )
}

fn build_semantic_searching_params_with_prefix() -> SearchingParams {
    SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Semantic(
            SemanticSearchingParamsBuilder::default()
                .query("query".to_string())
                .knn_amount(10)
                .min_score(None)
                .prefix(Some(PREFIX_FILTER.to_string()))
                .build()
                .expect("build semantic params failed"),
        ),
        build_result_params(),
        None,
    )
}

fn build_hybrid_searching_params_with_prefix() -> SearchingParams {
    SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Hybrid(
            HybridSearchingParamsBuilder::default()
                .query("query".to_string())
                .knn_amount(10)
                .min_score(None)
                .prefix(Some(PREFIX_FILTER.to_string()))
                .build()
                .expect("build hybrid params failed"),
        ),
        build_result_params(),
        None,
    )
}

#[rstest]
#[case(build_fulltext_searching_params_with_prefix(), "/query/bool/must")]
#[case(build_semantic_searching_params_with_prefix(), "/query/bool/filter")]
#[case(
    build_hybrid_searching_params_with_prefix(),
    "/query/hybrid/queries/1/bool/filter"
)]
fn test_build_search_query_forwards_prefix(
    #[case] params: SearchingParams,
    #[case] filter_path: &str,
) -> anyhow::Result<()> {
    let query = build_search_query(params)?;
    let filter = query
        .pointer(filter_path)
        .and_then(Value::as_array)
        .expect("expected an array at the given path");
    assert!(filter.contains(&json!({ "match": { "file_path": PREFIX_FILTER } })));
    Ok(())
}
