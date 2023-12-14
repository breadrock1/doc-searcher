use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::wrappers::bucket::Bucket;
use crate::wrappers::document::{Document, HighlightEntity};
use crate::wrappers::search_params::*;

use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::response::Response;
use elasticsearch::{BulkParts, Elasticsearch, SearchParts};
use hasher::{gen_hash, HashType};
use serde::Deserialize;
use serde_json::{json, Value};

use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::os::unix::prelude::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::string::ToString;
use tokio::sync::RwLockReadGuard;

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
            \"document_created\": { \"type\": \"string\" },
            \"document_modified\": { \"type\": \"string\" }
        }
    }
    ",
    )
}

pub async fn search_documents(
    elastic: &Elasticsearch,
    indexes: &[&str],
    body_value: &Value,
    es_params: &SearchParams,
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

pub fn build_search_query(parameters: &SearchParams) -> Value {
    let doc_size_to = parameters.document_size_to;
    let doc_size_from = parameters.document_size_from;
    let doc_cr_to = parameters.created_date_to.as_str();
    let doc_cr_from = parameters.created_date_from.as_str();
    let doc_ext = parameters.document_extension.as_str();
    let doc_path = parameters.document_path.as_str();
    let doc_type = parameters.document_type.as_str();

    let common_filter = CommonFilter::new()
        .with_date::<FilterRange, CreateDateQuery>("document_created", doc_cr_from, doc_cr_to)
        .with_range::<FilterRange>("document_size", doc_size_from, doc_size_to)
        .with_term::<FilterTerm>("document_extension", doc_ext)
        .with_term::<FilterTerm>("document_path", doc_path)
        .with_term::<FilterTerm>("document_type", doc_type)
        .build();

    let match_query = MultiMatchQuery::new(parameters.query.as_str());

    json!({
        "query": {
            "bool": {
                "must": match_query,
                "filter": common_filter
            }
        },
        "highlight" : {
            "order": "score",
            "fields" : {
                "entity_data": {
                    "pre_tags" : [""],
                    "post_tags" : [""]
                }
            }
        }
    })
}

pub fn build_search_similar_query(parameters: &SearchParams) -> Value {
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

    let built_bucket = BucketBuilder::default()
        .health(health.to_string())
        .status(status.to_string())
        .index(bucket_id.to_string())
        .uuid(uuid.to_string())
        .docs_count(docs_count.to_string())
        .docs_deleted(docs_deleted.to_string())
        .store_size(store_size.to_string())
        .pri_store_size(pri_store_size.to_string())
        .pri(None)
        .rep(None)
        .build();

    Ok(built_bucket.unwrap())
}

pub fn deserialize_document(document_ref: &Document) -> Result<Value, WebError> {
    match serde_json::to_value(document_ref) {
        Ok(value) => Ok(value),
        Err(err) => Err(WebError::DocumentSerializing(err.to_string())),
    }
}

pub fn load_directory_entity(directory: &Path) -> Vec<Document> {
    file_loader::load_directory_entity(directory)
        .into_iter()
        .map(Document::from)
        .collect::<Vec<Document>>()
}

pub async fn send_document(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    doc_form: &Document,
    bucket_id: &str,
) -> SendDocumentStatus {
    let to_value_result = serde_json::to_value(doc_form);
    let document_json = to_value_result.unwrap();
    let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

    body.push(json!({"index": { "_id": doc_form.document_md5_hash.as_str() }}).into());
    body.push(document_json.into());

    let response_result = elastic
        .bulk(BulkParts::Index(bucket_id))
        .body(body)
        .send()
        .await;

    match response_result {
        Ok(_) => SendDocumentStatus::new(true, doc_form.document_path.as_str()),
        Err(err) => {
            let err_msg = format!("Failed while loading file: {:?}", err);
            SendDocumentStatus::new(false, err_msg.as_str())
        }
    }
}

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn load_directory_entity_test() {
        let file_path = "/Users/breadrock/Downloads/optikot-data/tests-dockers";
        let path_object = Path::new(file_path);
        let _documents = load_directory_entity(&path_object);
        println!("{:?}", "sdf");
    }

    #[test]
    fn load_file_entity_test() {
        let file_path = "/Users/breadrock/Downloads/optikot-data/tests-dockers/fuzzer-configs/asa5516/asa5516-dhcp.py";
        let path_object = Path::new(file_path);
        let _documents = load_directory_entity(&path_object);
        println!("{:?}", "sdf");
    }
}
