use crate::errors::{WebError, WebResponse};
use crate::wrappers::bucket::Bucket;
use crate::wrappers::document::{Document, HighlightEntity};
use crate::wrappers::search_params::*;

use actix_web::web;
use elasticsearch::http::response::Response;
use elasticsearch::{Elasticsearch, SearchParts};
use serde::Deserialize;
use serde_json::{json, Value};
use std::string::ToString;

pub fn create_bucket_scheme() -> String {
    String::from(
        "
    {
        \"_source\": { \"enabled\": false },
        \"properties\": {
            \"bucket_uuid\": { \"type\": \"string\" },
            \"bucket_path\": { \"type\": \"string\" },
            \"document_name\": { \"type\": \"string\" },
            \"document_path\": { \"type\": \"string\" },
            \"document_size\": { \"type\": \"integer\" },
            \"document_type\": { \"type\": \"string\" },
            \"document_extension\": { \"type\": \"string\" },
            \"document_permissions\": { \"type\": \"integer\" },
            \"document_created\": { \"type\": \"date\" },
            \"document_modified\": { \"type\": \"date\" },
            \"document_md5_hash\": { \"type\": \"string\" },
            \"document_ssdeep_hash\": { \"type\": \"string\" },
            \"entity_data\": { \"type\": \"string\" },
            \"entity_keywords\": [],
        }
    }
    ",
    )
}

pub async fn search_documents(
    elastic: &Elasticsearch,
    indexes: &[&str],
    body_value: &Value,
    es_params: &SearchParameters,
) -> WebResponse<web::Json<Vec<Document>>> {
    let result_size = es_params.result_size;
    let result_offset = es_params.result_offset;
    let response_result = elastic
        .search(SearchParts::Index(indexes))
        .from(result_offset)
        .size(result_size)
        .body(body_value)
        .pretty(true)
        .allow_no_indices(true)
        .send()
        .await;

    match response_result {
        Err(err) => {
            let web_err = WebError::SearchFailed(err.to_string());
            Err(web_err)
        }
        Ok(response) => {
            let documents = parse_search_result(response).await;
            Ok(web::Json(documents))
        }
    }
}

pub async fn parse_search_result(response: Response) -> Vec<Document> {
    let common_object = response.json::<Value>().await.unwrap();
    let document_json = &common_object[&"hits"][&"hits"];
    let own_document = document_json.to_owned();
    let default_vec: Vec<Value> = Vec::default();
    let json_array = own_document.as_array().unwrap_or(&default_vec);

    json_array
        .iter()
        .map(parse_document_highlight)
        .map(Result::ok)
        .filter(Option::is_some)
        .flatten()
        .collect()
}

pub fn parse_document_highlight(value: &Value) -> Result<Document, serde_json::Error> {
    let source_value = value[&"_source"].to_owned();
    let mut document = Document::deserialize(source_value)?;

    let highlight_value = value[&"highlight"].to_owned();
    let highlight_entity = HighlightEntity::deserialize(highlight_value).ok();

    document.append_highlight(highlight_entity);
    Ok(document)
}

pub fn build_search_query(parameters: &SearchParameters) -> Value {
    let doc_size_to = parameters.document_size_to;
    let doc_size_from = parameters.document_size_from;
    let doc_size_query = DocumentSizeQuery::new(doc_size_from, doc_size_to);
    let doc_size_value = serde_json::to_value(doc_size_query).unwrap();

    let common_filter = json!({
        "bool": {
            "must": {
                "range": {
                    "document_size": doc_size_value,
                },
            },
            "should": {
                "term": {
                    "document_type": "*",
                    "document_path": "*",
                    "document_extension": "*",
                }
            }
        }
    });

    let query_str = QueryString::new(parameters.query.clone());
    let match_query = MultiMatchQuery::new(query_str);
    let match_value = serde_json::to_value(match_query).unwrap();

    json!({
        "query": {
            "bool": {
                "must": match_value,
                "filter": common_filter
            }
        },
        "highlight" : {
            "order": "score",
            "fields" : {
                "body" : {
                    "pre_tags" : [""],
                    "post_tags" : [""]
                },
                "matched_fields": [
                    "entity_data"
                ],
            }
        }
    })
}

pub fn build_search_similar_query(parameters: &SearchParameters) -> Value {
    let ssdeep_hash = &parameters.query;
    println!("Need find by this: {:?}", ssdeep_hash);
    json!({
        "query": {
            "more_like_this" : {
                "fields" : [
                    "entity_data",
                    "document_ssdeep_hash",
                ],
                "like" : ssdeep_hash,
                "min_doc_freq": 1,
                "min_term_freq" : 1,
                "max_query_terms" : 25,
            }
        }
    })
}

pub fn extract_bucket_stats(value: &Value) -> Result<Bucket, WebError> {
    let indicies = &value[&"indices"];
    let bucket_id = indicies.as_object();
    if bucket_id.is_none() {
        let msg = "There is no passed bucket name in json.";
        return Err(WebError::GetBucket(msg.to_string()));
    }

    let bucket_id = bucket_id.unwrap().keys().next().unwrap();
    let bucket = &indicies[bucket_id.as_str()];
    let health = &bucket[&"health"].as_str().unwrap();
    let status = &bucket[&"status"].as_str().unwrap();
    let uuid = &bucket[&"uuid"].as_str().unwrap();

    let primaries = &value[&"_all"][&"primaries"];
    let docs_count = &primaries[&"docs"][&"count"].as_i64().unwrap();
    let docs_deleted = &primaries[&"docs"][&"deleted"].as_i64().unwrap();
    let store_size = &primaries[&"store"][&"size_in_bytes"].as_i64().unwrap();
    let pri_store_size = &primaries[&"store"][&"total_data_set_size_in_bytes"]
        .as_i64()
        .unwrap();

    Ok(Bucket::new(
        health.to_string(),
        status.to_string(),
        bucket_id.to_string(),
        uuid.to_string(),
        docs_count.to_string(),
        docs_deleted.to_string(),
        store_size.to_string(),
        pri_store_size.to_string(),
        None,
        None,
    ))
}

pub fn deserialize_document(document_ref: &Document) -> Result<Value, WebError> {
    match serde_json::to_value(document_ref) {
        Ok(value) => Ok(value),
        Err(err) => Err(WebError::DocumentSerializing(err.to_string())),
    }
}
