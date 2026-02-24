use anyhow::Context;
use rstest::*;
use serde_json::{Value, json};

use crate::application::tests::fixture::search_params::*;
use crate::domain::searcher::models::{
    FullTextSearchingParams, HybridSearchingParams, RetrieveIndexDocumentsParams,
    SemanticSearchingParams,
};
use crate::domain::searcher::tests::fixture::params::build_filter_searching_params;
use crate::infrastructure::osearch::dto::{
    FullTextQueryParamsBuilder, HybridQueryParamsBuilder, RetrieveIndexDocsQueryParamsBuilder,
    SemanticQueryParamsBuilder,
};
use crate::infrastructure::osearch::query::QueryBuildHelper;

const RETRIEVE_FULL_PARAMS: &[u8] = include_bytes!("resources/retrieve-full-query.json");
const RETRIEVE_SIMPLE_PARAMS: &[u8] = include_bytes!("resources/retrieve-simple-query.json");
const FULLTEXT_FULL_PARAMS: &[u8] = include_bytes!("resources/fulltext-full-query.json");
const FULLTEXT_SIMPLE_PARAMS: &[u8] = include_bytes!("resources/fulltext-simple-query.json");
const SEMANTIC_FULL_PARAMS: &[u8] = include_bytes!("resources/semantic-full-query.json");
const SEMANTIC_SIMPLE_PARAMS: &[u8] = include_bytes!("resources/semantic-simple-query.json");
const SEMANTIC_TOKENS_PARAMS: &[u8] = include_bytes!("resources/semantic-tokens-query.json");
const HYBRID_FULL_PARAMS: &[u8] = include_bytes!("resources/hybrid-full-query.json");
const HYBRID_SIMPLE_PARAMS: &[u8] = include_bytes!("resources/hybrid-simple-query.json");

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
        .tokens(params.tokens)
        .model_id(params.model_id.unwrap_or_default())
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
        .tokens(params.tokens)
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .model_id(params.model_id.unwrap_or_default())
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
        .tokens(params.tokens)
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .model_id(params.model_id.unwrap_or_default())
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
        .model_id(params.model_id.unwrap_or_default())
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .result(result.to_owned())
        .filter(None)
        .build()
        .context("failed to build hybrid query params")?;

    let query = query_params.build_query();

    #[allow(unused_mut)]
    let mut comparable_query = serde_json::from_slice::<Value>(HYBRID_SIMPLE_PARAMS)?;

    #[cfg(feature = "support-opensearch-v3")]
    {
        comparable_query["query"]["hybrid"] = json!({"pagination_depth": 20});
    }

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
        .model_id(params.model_id.unwrap_or_default())
        .knn_amount(params.knn_amount)
        .min_score(params.min_score)
        .result(result.to_owned())
        .filter(Some(filter))
        .build()
        .context("failed to build hybrid query params")?;

    let query = query_params.build_query();

    #[allow(unused_mut)]
    let mut comparable_query = serde_json::from_slice::<Value>(HYBRID_FULL_PARAMS)?;

    #[cfg(feature = "support-opensearch-v3")]
    {
        comparable_query["query"]["hybrid"] = json!({"pagination_depth": 20});
    }

    assert_eq!(query, comparable_query);

    Ok(())
}
