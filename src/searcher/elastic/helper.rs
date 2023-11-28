use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::wrappers::bucket::Bucket;
use crate::wrappers::document::{Document, HighlightEntity};
use crate::wrappers::search_params::*;

use actix_web::{web, HttpResponse, Responder, ResponseError};
use chrono::{DateTime, Utc};
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::response::Response;
use elasticsearch::{BulkParts, Elasticsearch, SearchParts};
use futures::stream::{Stream, StreamExt};
use hasher::{gen_hash, HashType};
use serde::Deserialize;
use serde_json::{json, Value};

use futures::SinkExt;
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
    let doc_size_query = DocumentSizeQuery::new(doc_size_from, doc_size_to);
    let doc_size_value = serde_json::to_value(doc_size_query).unwrap();

    let mut common_filter = json!({
        "bool": {
            "must": [
                {
                    "range": {
                        "document_size": doc_size_value,
                    },
                }
            ]
        }
    });

    if (!parameters.document_extension.is_empty()) {
        let doc_ext = parameters.document_extension.as_str();
        let mut must_field = common_filter["bool"]["must"].as_array_mut().unwrap();
        must_field.push(json!({
            "term": {
                "document_extension": doc_ext
            }
        }));
    }

    if (!parameters.document_path.is_empty()) {
        let doc_path = parameters.document_path.as_str();
        let mut must_field = common_filter["bool"]["must"].as_array_mut().unwrap();
        must_field.push(json!({
            "term": {
                "document_path": doc_path
            }
        }));
    }

    if (!parameters.document_type.is_empty()) {
        let doc_type = parameters.document_type.as_str();
        let mut must_field = common_filter["bool"]["must"].as_array_mut().unwrap();
        must_field.push(json!({
            "term": {
                "document_type": doc_type
            }
        }));
    }

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

fn load_target_file(file_path: &Path) -> std::io::Result<Document> {
    let mut file = File::open(file_path)?;
    let file_metadata = file.metadata()?;
    let perms_ = file_metadata.permissions().mode();

    let file_path_ = file_path.to_str().unwrap_or("unknown");
    let file_name_ = file_path
        .file_name()
        .unwrap_or(OsStr::new(file_path_))
        .to_str()
        .unwrap_or("unknown");

    let ext_ = file_path
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("unknown");

    let mut file_data_ = String::new();
    file.read_to_string(&mut file_data_).unwrap_or_default();
    let md5_hash = gen_hash(HashType::MD5, file_data_.as_bytes());
    let binding = md5_hash.unwrap();
    let md5_hash_ = binding.get_hash_data();

    let ssdeep_hash = gen_hash(HashType::SSDEEP, file_data_.as_bytes());
    let binding = ssdeep_hash.unwrap();
    let ssdeep_hash_ = binding.get_hash_data();

    let created_ = file_metadata.created()?;
    let dt_cr_utc: DateTime<Utc> = created_.clone().into();

    let modified_ = file_metadata.modified()?;
    let dt_md_utc: DateTime<Utc> = modified_.clone().into();

    Ok(Document::create(
        "common_bucket".to_string(),
        "/".to_string(),
        file_name_.to_string(),
        file_path_.to_string(),
        file_metadata.size() as i32,
        "document".to_string(),
        ext_.to_string(),
        perms_ as i32,
        md5_hash_.to_string(),
        ssdeep_hash_.to_string(),
        file_data_,
        Vec::<String>::default(),
        Option::<HighlightEntity>::None,
        Some(dt_cr_utc),
        Some(dt_md_utc),
    ))
}

pub fn load_directory_entity(directory: &Path) -> Vec<Document> {
    if (directory.is_file()) {
        let loaded_result = load_target_file(&directory);
        return match loaded_result {
            Ok(document) => vec![document],
            Err(_) => Vec::default(),
        };
    }

    walkdir::WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| load_target_file(e.path()).ok())
        .collect()
}

pub async fn send_document(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    doc_form: &Document,
    bucket_id: &str,
) -> HttpResponse {
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

    if (response_result.is_err()) {
        println!("{:?}", response_result.err());
        println!("{:?}", "");
    }
    SuccessfulResponse::ok_response("Ok")
    // match response_result {
    //     Ok(_) => SuccessfulResponse::ok_response("Ok"),
    //     Err(err) => WebError::CreateDocument(err.to_string()).error_response(),
    // }
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
