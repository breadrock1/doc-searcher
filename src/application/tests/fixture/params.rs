use rstest::fixture;
use serde_json::{json, Value};

use crate::application::structures::params::{FilterParams, FilterParamsBuilder};
use crate::application::structures::params::{FullTextSearchParams, FullTextSearchParamsBuilder};
use crate::application::structures::params::{HybridSearchParams, HybridSearchParamsBuilder};
use crate::application::structures::params::{ResultParams, ResultParamsBuilder};
use crate::application::structures::params::{
    RetrieveDocumentParams, RetrieveDocumentParamsBuilder,
};
use crate::application::structures::params::{SemanticSearchParams, SemanticSearchParamsBuilder};

pub const QUERY_PARAMETER: &str = "There is some query";
pub const INDEXES_PARAMETER: &str = "test-folder";
pub const DOCUMENT_PATH: &str = "./test-document.docx";
pub const SEARCH_MODEL_ID: &str = "p30o65gBnrvKdVIONWdC";
pub const CURRENT_TIMESTAMP: i64 = 1756498133;
pub const KNN_AMOUNT: u16 = 1024;
pub const SEARCH_TOKENS: &[f64] = &[-1.4354, 0.435435];

pub fn build_result_params() -> ResultParams {
    ResultParamsBuilder::default()
        .order("desc".to_string())
        .size(10)
        .offset(0)
        .include_extra_fields(Some(true))
        .highlight_items(None)
        .highlight_item_size(None)
        .build()
        .unwrap()
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
        .unwrap()
}

pub fn build_comparable_query() -> Value {
    json!({
        "_source": {
            "exclude": [
                "chunked_text",
                "embeddings"
            ],
        },
        "query": {
            "bool": {
                "filter": [],
                "must": [{ "match_all": {} }],
            }
        },
        "sort": [
            {
                "created_at": { "order": "desc" }
            }
        ]
    })
}

#[fixture]
pub fn build_simple_retrieve_params() -> RetrieveDocumentParams {
    let result_params = build_result_params();
    RetrieveDocumentParamsBuilder::default()
        .path(None)
        .filter(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
pub fn build_with_path_retrieve_params() -> RetrieveDocumentParams {
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
pub fn build_with_filter_retrieve_params() -> RetrieveDocumentParams {
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
pub fn build_full_retrieve_params() -> RetrieveDocumentParams {
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
pub fn build_simple_fulltext_params() -> FullTextSearchParams {
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
pub fn build_with_query_fulltext_params() -> FullTextSearchParams {
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
pub fn build_with_filter_fulltext_params() -> FullTextSearchParams {
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
pub fn build_full_fulltext_params() -> FullTextSearchParams {
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
pub fn build_simple_semantic_params() -> SemanticSearchParams {
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
pub fn build_with_filter_semantic_params() -> SemanticSearchParams {
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
pub fn build_with_tokens_semantic_params() -> SemanticSearchParams {
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
pub fn build_full_semantic_params() -> SemanticSearchParams {
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
pub fn build_simple_hybrid_params() -> HybridSearchParams {
    let query = QUERY_PARAMETER.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let result_params = build_result_params();
    HybridSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(None)
        .knn_amount(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
pub fn build_with_filter_hybrid_params() -> HybridSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let result_params = build_result_params();
    HybridSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(None)
        .knn_amount(None)
        .result(result_params)
        .build()
        .unwrap()
}

#[fixture]
pub fn build_full_hybrid_params() -> HybridSearchParams {
    let query = DOCUMENT_PATH.to_string();
    let indexes = INDEXES_PARAMETER.to_string();
    let result_params = build_result_params();
    HybridSearchParamsBuilder::default()
        .query(query)
        .indexes(indexes)
        .model_id(Some(SEARCH_MODEL_ID.to_string()))
        .knn_amount(Some(KNN_AMOUNT))
        .result(result_params)
        .build()
        .unwrap()
}
