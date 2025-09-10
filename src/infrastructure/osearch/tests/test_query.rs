use rstest::*;
use serde_json::{json, Value};

use crate::application::structures::params::*;
use crate::infrastructure::osearch::query::{QueryBuilder, QueryBuilderParams};

const QUERY_PARAMETER: &str = "There is some query";
const INDEXES_PARAMETER: &str = "test-folder";
const DOCUMENT_PATH: &str = "./test-document.docx";
const SEARCH_MODEL_ID: &str = "p30o65gBnrvKdVIONWdC";
const CURRENT_TIMESTAMP: i64 = 1756498133;
const KNN_AMOUNT: u16 = 1024;
const SEARCH_TOKENS: &[f64] = &[-1.4354, 0.435435];

fn build_result_params() -> ResultParams {
    ResultParamsBuilder::default()
        .order("desc".to_string())
        .size(10)
        .offset(0)
        .include_extra_fields(Some(true))
        .build()
        .unwrap()
}

fn build_filter_params() -> FilterParams {
    FilterParamsBuilder::default()
        .size_from(Some(0))
        .size_to(Some(4096))
        .created_from(Some(CURRENT_TIMESTAMP))
        .created_to(Some(CURRENT_TIMESTAMP))
        .modified_from(Some(CURRENT_TIMESTAMP))
        .modified_to(Some(CURRENT_TIMESTAMP))
        .build()
        .unwrap()
}

fn build_comparable_query() -> Value {
    json!({
        "_source": {
            "exclude": [
                "chunked_text",
                "embeddings"
            ],
        },
        "highlight": {
            "fields": {
                "content": {
                    "type": "plain",
                    "pre_tags": [""],
                    "post_tags": [""]
                }
            }
        },
        "query": {
            "bool": {
                "filter": [],
                "must": [
                    {
                        "match_all": {}
                    }
                ],
            }
        },
        "sort": [
            {
                "created_at": {
                    "order": "desc"
                }
            }
        ],
    })
}

#[fixture]
fn build_simple_retrieve_params() -> RetrieveDocumentParams {
    let result_params = build_result_params();
    RetrieveDocumentParamsBuilder::default()
        .path(None)
        .filter(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_with_path_retrieve_params() -> RetrieveDocumentParams {
    let result_params = build_result_params();
    let path = DOCUMENT_PATH.to_string();
    RetrieveDocumentParamsBuilder::default()
        .path(Some(path))
        .filter(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_with_filter_retrieve_params() -> RetrieveDocumentParams {
    let filter_params = build_filter_params();
    let result_params = build_result_params();
    RetrieveDocumentParamsBuilder::default()
        .path(None)
        .filter(Some(filter_params))
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_full_retrieve_params() -> RetrieveDocumentParams {
    let path = DOCUMENT_PATH.to_string();
    let filter_params = build_filter_params();
    let result_params = build_result_params();
    RetrieveDocumentParamsBuilder::default()
        .path(Some(path))
        .filter(Some(filter_params))
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_simple_fulltext_params() -> FullTextSearchParams {
    let result_params = build_result_params();
    let indexes = INDEXES_PARAMETER.to_string();
    FullTextSearchParamsBuilder::default()
        .query(None)
        .indexes(indexes)
        .filter(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_with_query_fulltext_params() -> FullTextSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let result_params = build_result_params();
    FullTextSearchParamsBuilder::default()
        .query(Some(query))
        .indexes(indexes)
        .filter(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_with_filter_fulltext_params() -> FullTextSearchParams {
    let filter_params = build_filter_params();
    let result_params = build_result_params();
    FullTextSearchParamsBuilder::default()
        .query(None)
        .filter(Some(filter_params))
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_full_fulltext_params() -> FullTextSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let filter_params = build_filter_params();
    let result_params = build_result_params();
    FullTextSearchParamsBuilder::default()
        .query(Some(query))
        .filter(Some(filter_params))
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_simple_semantic_params() -> SemanticSearchParams {
    let query = QUERY_PARAMETER.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let result_params = build_result_params();
    SemanticSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(None)
        .tokens(None)
        .knn_amount(None)
        .filter(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_with_filter_semantic_params() -> SemanticSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let filter_params = build_filter_params();
    let result_params = build_result_params();
    SemanticSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(None)
        .tokens(None)
        .knn_amount(None)
        .filter(Some(filter_params))
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_with_tokens_semantic_params() -> SemanticSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let result_params = build_result_params();
    SemanticSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(None)
        .tokens(Some(SEARCH_TOKENS.to_vec()))
        .knn_amount(None)
        .filter(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_full_semantic_params() -> SemanticSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let filter_params = build_filter_params();
    let result_params = build_result_params();
    SemanticSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(Some(SEARCH_MODEL_ID.to_string()))
        .tokens(None)
        .knn_amount(Some(KNN_AMOUNT))
        .filter(Some(filter_params))
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_simple_hybrid_params() -> HybridSearchParams {
    let query = QUERY_PARAMETER.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let result_params = build_result_params();
    HybridSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(None)
        .knn_amount(None)
        .filter(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_with_filter_hybrid_params() -> HybridSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let filter_params = build_filter_params();
    let result_params = build_result_params();
    HybridSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(None)
        .knn_amount(None)
        .filter(Some(filter_params))
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
fn build_full_hybrid_params() -> HybridSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let filter_params = build_filter_params();
    let result_params = build_result_params();
    HybridSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(Some(SEARCH_MODEL_ID.to_string()))
        .knn_amount(Some(KNN_AMOUNT))
        .filter(Some(filter_params))
        .result(result_params)
        .build()
        .unwrap()
}

#[rstest]
fn test_build_simplest_query_from_retrieve_params(
    #[from(build_simple_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let compare_query = build_comparable_query();
    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_with_path_query_from_retrieve_params(
    #[from(build_with_path_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["query"] = json!({
        "query": {
            "bool": {
                "filter": [],
                "must": [
                    {
                        "match": {
                            "file_path": DOCUMENT_PATH,
                        }
                    }
                ],
            }
        },
    });

    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_with_filter_query_from_retrieve_params(
    #[from(build_with_filter_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["query"]["bool"]["filter"] = json!([
        {
            "range": {
                "created_at": {
                    "gte": CURRENT_TIMESTAMP,
                    "lte": CURRENT_TIMESTAMP,
                }
            }
        },
        {
            "range": {
                "file_size": {
                    "gte": 0,
                    "lte": 4096,
                }
            }
        },
    ]);

    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_simple_fulltext_search_params(
    #[from(build_simple_fulltext_params)] params: FullTextSearchParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let compare_query = build_comparable_query();
    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_with_path_query_from_fulltext_params(
    #[from(build_with_path_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["query"] = json!({
        "bool": {
            "filter": [],
            "must": [
                {
                    "match": {
                        "file_path": DOCUMENT_PATH,
                    }
                }
            ],
        }
    });

    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_with_filter_query_from_fulltext_params(
    #[from(build_with_filter_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["query"]["bool"]["filter"] = json!([
        {
            "range": {
                "created_at": {
                    "gte": CURRENT_TIMESTAMP,
                    "lte": CURRENT_TIMESTAMP,
                }
            }
        },
        {
            "range": {
                "file_size": {
                    "gte": 0,
                    "lte": 4096,
                }
            }
        },
    ]);

    println!("{:?}", compare_query);
    assert_eq!(query, compare_query);

    Ok(())
}
