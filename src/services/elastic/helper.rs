use crate::errors::{JsonResponse, WebError};
use crate::services::elastic::send_status::SendDocumentStatus;

use elquery::exclude_fields::ExcludeFields;
use elquery::filter_query::{CommonFilter, CreateDateQuery, FilterRange, FilterTerm};
use elquery::highlight_query::HighlightOrder;
use elquery::search_query::MultiMatchQuery;
use elquery::similar_query::SimilarQuery;
use wrappers::bucket::{Bucket, BucketBuilder};
use wrappers::document::{Document, HighlightEntity};
use wrappers::schema::BucketSchema;
use wrappers::scroll::PagintatedResult;
use wrappers::search_params::SearchParams;

use actix_web::web;
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::response::Response;
use elasticsearch::{BulkParts, CountParts, Elasticsearch, SearchParts};
use serde::Deserialize;
use serde_json::{json, Value};
use std::string::ToString;
use tokio::sync::RwLockReadGuard;

pub async fn send_document(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    doc_form: &Document,
    bucket_id: &str,
) -> SendDocumentStatus {
    let document_id = doc_form.content_md5.as_str();

    let to_value_result = serde_json::to_value(doc_form);
    let document_json = to_value_result.unwrap();
    let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

    body.push(json!({"index": { "_id": document_id }}).into());
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

pub async fn check_duplication(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    bucket_id: &str,
    document_id: &str,
) -> bool {
    let response_result = elastic
        .count(CountParts::Index(&[bucket_id]))
        .body(json!({
                "query" : {
                    "term" : {
                        "content_md5" : document_id
                    }
                }
        }))
        .send()
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        log::error!("Failed while checking duplicate: {}", err);
        return false;
    }

    let response = response_result.unwrap();
    let serialize_result = response.json::<Value>().await;
    match serialize_result {
        Ok(value) => {
            let count = value["count"].as_i64().unwrap_or(0);
            count > 0
        }
        Err(err) => {
            log::error!("Failed to check duplicate for {}: {}", document_id, err);
            false
        }
    }
}

pub async fn search_documents(
    elastic: &Elasticsearch,
    indexes: &[&str],
    body_value: &Value,
    es_params: &SearchParams,
) -> JsonResponse<PagintatedResult<Vec<Document>>> {
    let result_size = es_params.result_size;
    let result_offset = es_params.result_offset;
    let response_result = elastic
        .search(SearchParts::Index(indexes))
        .from(result_offset)
        .size(result_size)
        .body(body_value)
        .pretty(true)
        .scroll(es_params.get_scroll())
        .allow_no_indices(true)
        .send()
        .await;

    match response_result {
        Err(err) => Err(WebError::SearchFailed(err.to_string())),
        Ok(response) => {
            let documents = parse_search_result(response).await;
            Ok(web::Json(documents))
        }
    }
}

pub async fn parse_search_result(response: Response) -> PagintatedResult<Vec<Document>> {
    let common_object = response.json::<Value>().await.unwrap();
    let document_json = &common_object[&"hits"][&"hits"];
    let scroll_id = common_object[&"_scroll_id"]
        .as_str()
        .map_or_else(|| None, |x| Some(x.to_string()));

    let own_document = document_json.to_owned();
    let default_vec: Vec<Value> = Vec::default();
    let json_array = own_document.as_array().unwrap_or(&default_vec);

    let founded_documents = json_array
        .iter()
        .map(parse_document_highlight)
        .map(Result::ok)
        .filter(Option::is_some)
        .flatten()
        .collect::<Vec<Document>>();

    PagintatedResult::new_with_opt_id(founded_documents, scroll_id)
}

fn parse_document_highlight(value: &Value) -> Result<Document, serde_json::Error> {
    let source_value = value[&"_source"].to_owned();
    let document_result = Document::deserialize(source_value);
    if document_result.is_err() {
        let err = document_result.err().unwrap();
        log::error!("Failed while deserialize doc: {}", err);
        return Err(err);
    }

    let mut document = document_result?;
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
    let doc_type = parameters.document_type.as_str();

    let common_filter = CommonFilter::new()
        .with_date::<FilterRange, CreateDateQuery>("document_created", doc_cr_from, doc_cr_to)
        .with_range::<FilterRange>("document_size", doc_size_from, doc_size_to)
        .with_term::<FilterTerm>("document_extension", doc_ext)
        .with_term::<FilterTerm>("document_type", doc_type)
        .build();

    let match_query = MultiMatchQuery::new(parameters.query.as_str());
    let highlight_order = HighlightOrder::default();

    let cont_vector = Some(vec!["content_vector".to_string()]);
    let exclude_fields = ExcludeFields::new(cont_vector);

    let query_json_object = json!({
        "_source": exclude_fields,
        "query": {
            "bool": {
                "must": match_query,
                "filter": common_filter
            }
        },
        "highlight": highlight_order
    });

    // TODO: Implement generating cosine searching.
    #[cfg(feature = "enable-semantic")]
    if cfg!(feature = "enable-semantic") {
        query_json_object[&"test"] = json!({});
    }

    query_json_object
}

pub fn build_search_similar_query(parameters: &SearchParams) -> Value {
    let fields = vec!["entity_data".to_string(), "documen_ssdeep_hash".to_string()];
    let ssdeep_hash = &parameters.query;
    let similar_query = SimilarQuery::new(ssdeep_hash.clone(), fields);
    json!({ "query": similar_query })
}

pub fn extract_bucket_stats(value: &Value) -> Result<Bucket, WebError> {
    let indices = &value[&"indices"];
    let bucket_id = indices.as_object();
    if bucket_id.is_none() {
        let msg = "There is no passed bucket name in json.";
        return Err(WebError::GetBucket(msg.to_string()));
    }

    let bucket_id = bucket_id.unwrap().keys().next().unwrap();
    let bucket = &indices[bucket_id.as_str()];
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
        .docs_count(Some(docs_count.to_string()))
        .docs_deleted(Some(docs_deleted.to_string()))
        .store_size(Some(store_size.to_string()))
        .pri_store_size(Some(pri_store_size.to_string()))
        .pri(None)
        .rep(None)
        .build();

    Ok(built_bucket.unwrap())
}

pub fn create_bucket_scheme() -> String {
    let schema = BucketSchema::default();
    serde_json::to_string_pretty(&schema).unwrap()
}
