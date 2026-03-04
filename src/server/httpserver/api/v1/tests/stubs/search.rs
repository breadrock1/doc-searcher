use serde_json::json;
use serde_json::Value;

use doc_search_core::domain::searcher::models::{
    DocumentPartEntrails, FoundedDocument, FoundedDocumentBuilder,
};

use crate::server::httpserver::api::v1::schema::FoundedDocumentPartSchema;

use super::constants::{
    COMPOSITE_INDEX_IDS, DOCUMENT_CONTENT, DOCUMENT_CREATED_AT, DOCUMENT_FILE_NAME,
    DOCUMENT_FILE_PATH, DOCUMENT_FILE_SIZE, DOCUMENT_MODIFIED_AT, KNN_AMOUNT, LARGE_DOCUMENT_ID,
    MIN_SCORE, MODEL_ID, SCROLL_ID, SEARCH_QUERY, TEST_INDEX_ID,
};

const QUERY_TOKENS: [f64; 3] = [-1.123, 0.123, 1.123];
const RETRIEVE_FILE_PATH: &str = "./test-";
const RESULT_ORDER: &str = "ASC";
const RESULT_SIZE: u16 = 10;
const RESULT_OFFSET: u16 = 0;
const HIGHLIGHT_ITEMS_COUNT: u16 = 3;
const HIGHLIGHT_ITEMS_SIZE: u16 = 100;
const RESULT_INCLUDE_EXTRA_FIELDS: bool = true;

const FILTER_SIZE_FROM: u16 = 16;
const FILTER_SIZE_TO: u16 = 4096;
const FILTER_TIMESTAMP_FROM: i64 = 1750731615;
const FILTER_TIMESTAMP_TO: i64 = 1750731655;
const FILTER_SOURCE: &str = "source";
const FILTER_SEMANTIC_SOURCE: &str = "semantic source";
const FILTER_DISTANCE: &str = "10km";
const FILTER_DOC_CLASS: &str = "10km";
const FILTER_DOC_CLASS_PROBABILITY: f32 = 0.7;
const FILTER_LOCATION_COORDS: [f32; 2] = [0.0, 0.0];

pub fn pagination_result_json_object() -> Value {
    json!({
        "founded": [
            {
                "id": LARGE_DOCUMENT_ID,
                "index": TEST_INDEX_ID,
                "score": 0.76,
                "highlight": [
                    "Intuition is",
                    "very",
                    "important"
                ],
                "document": {
                    "large_doc_id": LARGE_DOCUMENT_ID,
                    "doc_part_id": 1,
                    "file_name": DOCUMENT_FILE_NAME,
                    "file_path": DOCUMENT_FILE_PATH,
                    "file_size": DOCUMENT_FILE_SIZE,
                    "content": DOCUMENT_CONTENT,
                    "created_at": DOCUMENT_CREATED_AT,
                    "modified_at": DOCUMENT_MODIFIED_AT,
                }
            },
            {
                "id": LARGE_DOCUMENT_ID,
                "index": TEST_INDEX_ID,
                "score": 0.76,
                "highlight": [
                    "Intuition is",
                    "very",
                    "important"
                ],
                "document": {
                    "large_doc_id": LARGE_DOCUMENT_ID,
                    "doc_part_id": 2,
                    "file_name": DOCUMENT_FILE_NAME,
                    "file_path": DOCUMENT_FILE_PATH,
                    "file_size": DOCUMENT_FILE_SIZE,
                    "content": DOCUMENT_CONTENT,
                    "created_at": DOCUMENT_CREATED_AT,
                    "modified_at": DOCUMENT_MODIFIED_AT,
                }
            },
        ],
        "scroll_id": SCROLL_ID,
    })
}

pub fn fulltext_search_params_json_object() -> Value {
    json!({
        "query": SEARCH_QUERY,
        "indexes": COMPOSITE_INDEX_IDS,
        "result": result_params_json_object(),
    })
}

pub fn fulltext_search_params_with_filter_json_object() -> Value {
    json!({
        "query": SEARCH_QUERY,
        "indexes": COMPOSITE_INDEX_IDS,
        "filter": filter_params_json_object(),
        "result": result_params_json_object(),
    })
}

pub fn semantic_search_params_json_object() -> Value {
    json!({
        "query": SEARCH_QUERY,
        "knn_amount": KNN_AMOUNT,
        "model_id": MODEL_ID,
        "indexes": COMPOSITE_INDEX_IDS,
        "result": result_params_json_object(),
    })
}

pub fn semantic_search_params_with_tokens_json_object() -> Value {
    json!({
        "query": SEARCH_QUERY,
        "knn_amount": KNN_AMOUNT,
        "model_id": MODEL_ID,
        "tokens": QUERY_TOKENS.to_vec(),
        "indexes": COMPOSITE_INDEX_IDS,
        "result": result_params_json_object(),
    })
}

pub fn semantic_search_params_with_filter_json_object() -> Value {
    json!({
        "query": SEARCH_QUERY,
        "knn_amount": KNN_AMOUNT,
        "model_id": MODEL_ID,
        "indexes": COMPOSITE_INDEX_IDS,
        "filter": filter_params_json_object(),
        "result": result_params_json_object(),
    })
}

pub fn hybrid_search_params_json_object() -> Value {
    json!({
        "query": SEARCH_QUERY,
        "knn_amount": KNN_AMOUNT,
        "model_id": MODEL_ID,
        "indexes": COMPOSITE_INDEX_IDS,
        "min_score": MIN_SCORE,
        "result": result_params_json_object(),
    })
}

pub fn hybrid_search_params_with_filter_json_object() -> Value {
    json!({
        "query": SEARCH_QUERY,
        "knn_amount": KNN_AMOUNT,
        "model_id": MODEL_ID,
        "indexes": COMPOSITE_INDEX_IDS,
        "min_score": MIN_SCORE,
        "filter": filter_params_json_object(),
        "result": result_params_json_object(),
    })
}

pub fn retrieve_index_documents_params_json_object() -> Value {
    json!({
        "path": RETRIEVE_FILE_PATH,
        "result": result_params_json_object(),
    })
}

pub fn retrieve_index_documents_params_with_filter_json_object() -> Value {
    let mut base_params = retrieve_index_documents_params_json_object();
    base_params["filter"] = filter_params_json_object();
    base_params
}

pub fn result_params_json_object() -> Value {
    json!({
        "order": RESULT_ORDER,
        "size": RESULT_SIZE,
        "offset": RESULT_OFFSET,
        "highlight_items": HIGHLIGHT_ITEMS_COUNT,
        "highlight_item_size": HIGHLIGHT_ITEMS_SIZE,
        "include_extra_fields": RESULT_INCLUDE_EXTRA_FIELDS,
    })
}

pub fn filter_params_json_object() -> Value {
    json!({
        "doc_part_id": 1,
        "pipeline_id": 1,
        "size_from": FILTER_SIZE_FROM,
        "size_to": FILTER_SIZE_TO,
        "created_from": FILTER_TIMESTAMP_FROM,
        "created_to": FILTER_TIMESTAMP_TO,
        "modified_from": FILTER_TIMESTAMP_FROM,
        "modified_to": FILTER_TIMESTAMP_TO,
        "source": FILTER_SOURCE,
        "semantic_source": FILTER_SEMANTIC_SOURCE,
        "distance": FILTER_DISTANCE,
        "location_coordinates": FILTER_LOCATION_COORDS,
        "document_class_probability": FILTER_DOC_CLASS_PROBABILITY,
        "document_class": FILTER_DOC_CLASS,
    })
}

pub fn document_part_entrails_with_part_id(doc_part_id: usize) -> DocumentPartEntrails {
    DocumentPartEntrails {
        large_doc_id: LARGE_DOCUMENT_ID.to_string(),
        doc_part_id,
        file_name: DOCUMENT_FILE_NAME.to_string(),
        file_path: DOCUMENT_FILE_PATH.to_string(),
        file_size: DOCUMENT_FILE_SIZE,
        created_at: DOCUMENT_CREATED_AT,
        modified_at: DOCUMENT_MODIFIED_AT,
        content: Some(DOCUMENT_CONTENT.to_string()),
        chunked_text: None,
        embeddings: None,
        metadata: None,
    }
}

pub fn founded_document_with_part_id(doc_part_id: usize) -> FoundedDocument {
    let highlight = DOCUMENT_CONTENT
        .split(' ')
        .map(String::from)
        .collect::<Vec<String>>();
    let doc_part_entrails = document_part_entrails_with_part_id(doc_part_id);
    FoundedDocumentBuilder::default()
        .id(LARGE_DOCUMENT_ID.to_string())
        .index(TEST_INDEX_ID.to_string())
        .score(Some(MIN_SCORE))
        .highlight(highlight)
        .document(doc_part_entrails)
        .build()
        .expect("build founded document fixture failed")
}
