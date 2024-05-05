use crate::errors::{PaginateJsonResponse, WebError};
use crate::services::elastic::send_status::SendDocumentStatus;

use elquery::exclude_fields::ExcludeFields;
use elquery::filter_query::{CommonFilter, CreateDateQuery, CreatedAtDateQuery};
use elquery::filter_query::{FilterMatch, FilterRange, FilterTerm};
use elquery::highlight_query::HighlightOrder;
use elquery::search_query::MultiMatchQuery;
use elquery::similar_query::SimilarQuery;
use wrappers::bucket::Folder;
use wrappers::document::{Document, DocumentPreview, HighlightEntity};
use wrappers::schema::BucketSchema;
use wrappers::scroll::PaginatedResult;
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

pub async fn send_document_preview(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    doc_form: &DocumentPreview,
    bucket_id: &str,
) -> SendDocumentStatus {
    let document_id = doc_form.id.as_str();

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
        Ok(_) => SendDocumentStatus::new(true, doc_form.location.as_str()),
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

pub async fn search_documents_preview(
    elastic: &Elasticsearch,
    indexes: &[&str],
    body_value: &Value,
    es_params: &SearchParams,
) -> PaginateJsonResponse<Vec<DocumentPreview>> {
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
                .map(|value| {
                    let source_value = &value[&"_source"];
                    match DocumentPreview::deserialize(source_value) {
                        Ok(docs) => Ok(DocumentPreview::from(docs)),
                        Err(err) => {
                            log::error!("Failed while deserialize doc: {}", err);
                            Err(err)
                        }
                    }
                })
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect::<Vec<DocumentPreview>>();

            Ok(web::Json(PaginatedResult::new_with_opt_id(
                founded_documents,
                scroll_id,
            )))
        }
    }
}

pub async fn search_documents(
    elastic: &Elasticsearch,
    indexes: &[&str],
    body_value: &Value,
    es_params: &SearchParams,
) -> PaginateJsonResponse<Vec<Document>> {
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

            #[cfg(feature = "enable-semantic")]
            if cfg!(feature = "enable-semantic") {
                let mut documents = documents;
                sort_by_cosine(es_params.get_query(), documents.get_founded_mut()).await;
                return Ok(web::Json(documents));
            }

            Ok(web::Json(documents))
        }
    }
}

pub async fn parse_search_result(response: Response) -> PaginatedResult<Vec<Document>> {
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

    PaginatedResult::new_with_opt_id(founded_documents, scroll_id)
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

pub fn build_match_all_query(parameters: &SearchParams) -> Value {
    let doc_cr_from = parameters.created_date_from.as_str();
    let query = parameters.query.as_str();
    let default_location = &String::default();
    let location = parameters
        .buckets
        .as_ref()
        .unwrap_or(default_location)
        .as_str();

    let common_filter = CommonFilter::new()
        .with_date::<FilterRange, CreatedAtDateQuery>("created_at", doc_cr_from, "")
        .with_match::<FilterMatch>("location", location)
        .with_match::<FilterMatch>("name", query)
        .build();

    json!({
        "query": {
            "bool": {
                "filter": common_filter,
                "must": {
                    "match_all": {}
                }
            }
        }
    })
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

    let mut query_json_object = json!({
        "query": {
            "bool": {
                "must": match_query,
                "filter": common_filter
            }
        },
        "highlight": highlight_order
    });

    if !cfg!(feature = "enable-semantic") {
        let cont_vector = Some(vec!["content_vector".to_string()]);
        let exclude_fields = ExcludeFields::new(cont_vector);
        let exclude_value = serde_json::to_value(exclude_fields).unwrap();
        query_json_object[&"_source"] = exclude_value;
    }

    query_json_object
}

pub fn build_search_similar_query(parameters: &SearchParams) -> Value {
    let fields = vec!["content".to_string(), "document_ssdeep".to_string()];
    let ssdeep_hash = &parameters.query;
    let similar_query = SimilarQuery::new(ssdeep_hash.clone(), fields);
    json!({ "query": similar_query })
}

pub fn extract_bucket_stats(value: &Value) -> Result<Folder, WebError> {
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

    let built_bucket = Folder::builder()
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

pub fn create_bucket_scheme(// is_preview: bool,
) -> String {
    // if is_preview {
    //     // {
    //     //   "id": "string",
    //     //   "name": "Перевозка груза МСК",
    //     //   "created_at": "2024-05-03",
    //     //   "updated_at": "2024-05-03",
    //     //   "quality_recognization": 10000,
    //     //   "file_size": 12545,
    //     //   "location": "string",
    //     //   "properties": [
    //     //     {
    //     //      "group_name": "Приём груза",
    //     //      "group_values": [
    //     //         {
    //     //          "key": "field_date_smgs",
    //     //          "name": "Дата и время (печатные)",
    //     //          "value": "18.03.2024, 23:59"
    //     //        }
    //     //      ]
    //     //    }
    //     //  ]
    //     // }
    //     let schema = json!({
    //         "id": {"type": "string"},
    //         "name": {"type": "string"},
    //         "created_at": {"type": "date"},
    //         "updated_at": {"type": "date"},
    //         "quality_recognition": {"type": "int"},
    //         "file_size": {"type": "int"},
    //         "location": {"type": "string"},
    //         "properties": {"type": "object"},
    //     });
    //     return serde_json::to_string_pretty(&schema).unwrap();
    // }
    let schema = BucketSchema::default();
    serde_json::to_string_pretty(&schema).unwrap()
}

#[cfg(feature = "enable-semantic")]
pub async fn sort_by_cosine(query: &str, documents: &mut [Document]) {
    use simsimd::SpatialSimilarity;

    let query_tokens_result = load_query_tokens(query).await;
    if query_tokens_result.is_err() {
        let err = query_tokens_result.err().unwrap();
        log::warn!("Failed while getting query tokens: {}", err);
        return;
    }

    let query_tokens = query_tokens_result.unwrap();
    documents.sort_by_key(
        |doc| match f64::cosine(&query_tokens, &doc.content_vector) {
            None => 0i32,
            Some(dist) => dist.cos() as i32,
        },
    );

    documents
        .iter_mut()
        .for_each(|doc| doc.content_vector = Vec::default());
}

#[cfg(feature = "enable-semantic")]
async fn load_query_tokens(query: &str) -> Result<Vec<f64>, anyhow::Error> {
    let embeddings_url = std::env::var("EMBEDDINGS_URL").unwrap_or_default();
    let client = reqwest::Client::new();
    let response = client
        .post(embeddings_url)
        .json(&json!({
            "inputs": query,
            "truncate": false
        }))
        .send()
        .await?;

    let query_tokens = response.json::<Vec<Vec<f64>>>().await?;
    match query_tokens.first() {
        None => {
            let msg = "Failed while sending request";
            Err(anyhow::Error::msg(msg.to_string()))
        }
        Some(vector) => Ok(vector.to_owned()),
    }
}
