use crate::errors::{PaginateJsonResponse, SuccessfulResponse, WebError};

use elquery::exclude_fields::ExcludeFields;
use elquery::filter_query::{CommonFilter, CreateDateQuery, CreatedAtDateQuery};
use elquery::filter_query::{FilterMatch, FilterRange, FilterTerm};
use elquery::highlight_query::HighlightOrder;
use elquery::search_query::MultiMatchQuery;
use elquery::similar_query::SimilarQuery;
use wrappers::document::{Document, DocumentPreview, HighlightEntity};
use wrappers::folder::Folder;
use wrappers::s_params::SearchParams;
use wrappers::scroll::PaginatedResult;
use wrappers::schema::{DocumentPreviewSchema, DocumentSchema};

use actix_web::web;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::response::Response;
use elasticsearch::http::Method;
use elasticsearch::{BulkParts, CountParts, Elasticsearch, SearchParts};
use serde::Deserialize;
use serde_json::{json, Value};
use std::string::ToString;
use tokio::sync::RwLockReadGuard;

pub(crate) async fn get_all_clusters(elastic: &Elasticsearch) -> Result<Response, WebError> {
    elastic
        .send(
            Method::Get,
            "/_cat/nodes",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await
        .map_err(WebError::from)
}

pub(crate) async fn store_document(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    doc_form: &Document,
    folder_id: &str,
) -> Result<SuccessfulResponse, WebError> {
    let document_id = doc_form.get_doc_id();

    let to_value_result = serde_json::to_value(doc_form);
    let document_json = to_value_result.unwrap();
    let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

    body.push(json!({"index": { "_id": document_id }}).into());
    body.push(document_json.into());

    let response = elastic
        .bulk(BulkParts::Index(folder_id))
        .body(body)
        .send()
        .await
        .map_err(WebError::from)?;

    parse_elastic_response(response).await
}

pub(crate) async fn store_doc_preview(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    doc_form: &DocumentPreview,
    folder_id: &str,
) -> Result<SuccessfulResponse, WebError> {
    let document_id = doc_form.id.as_str();

    let to_value_result = serde_json::to_value(doc_form);
    let document_json = to_value_result.unwrap();
    let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

    body.push(json!({"index": { "_id": document_id }}).into());
    body.push(document_json.into());

    let response = elastic
        .bulk(BulkParts::Index(folder_id))
        .body(body)
        .send()
        .await
        .map_err(WebError::from)?;

    parse_elastic_response(response).await
}

pub(crate) async fn parse_elastic_response(response: Response) -> Result<SuccessfulResponse, WebError> {
    if !response.status_code().is_success() {
        return Err(extract_exception(response).await);
    }

    Ok(SuccessfulResponse::success("Ok"))
}

pub(crate) async fn extract_exception(response: Response) -> WebError {
    let exception_opt = response.exception().await.map_err(WebError::from).unwrap();
    return match exception_opt {
        None => WebError::UnknownError("Unknown error".to_string()),
        Some(exception) => WebError::from(exception),
    };
}

pub(crate) async fn check_duplication(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    folder_id: &str,
    document_id: &str,
) -> Result<bool, WebError> {
    let response = elastic
        .count(CountParts::Index(&[folder_id]))
        .body(json!({
            "query" : {
                "term" : {
                    "content_md5" : document_id
                }
            }
        }))
        .send()
        .await
        .map_err(WebError::from)?;

    let result = response
        .json::<Value>()
        .await
        .map_or(false, |value| {
            let count = value["count"].as_i64().unwrap_or(0);
            count > 0
        });

    Ok(result)
}

pub(crate) async fn search_documents_preview(
    elastic: &Elasticsearch,
    es_params: &SearchParams,
    body_value: &Value,
    indexes: &[&str],
) -> PaginateJsonResponse<Vec<DocumentPreview>> {
    match send_search_request(elastic, es_params, body_value, indexes).await {
        Ok(response) => Ok(web::Json(extract_document_preview(response).await)),
        Err(err) => Err(WebError::from(err)),
    }
}

pub(crate) async fn search_documents(
    elastic: &Elasticsearch,
    es_params: &SearchParams,
    body_value: &Value,
    indexes: &[&str],
) -> PaginateJsonResponse<Vec<Document>> {
    match send_search_request(elastic, es_params, body_value, indexes).await {
        Ok(response) => Ok(web::Json(parse_search_result(response).await)),
        Err(err) => Err(WebError::SearchError(err.to_string())),
    }
}

async fn send_search_request(
    elastic: &Elasticsearch,
    es_params: &SearchParams,
    body_value: &Value,
    indexes: &[&str],
) -> Result<Response, elasticsearch::Error> {
    let (result_size, result_offset) = es_params.get_results_params();
    elastic
        .search(SearchParts::Index(indexes))
        .from(result_offset)
        .size(result_size)
        .body(body_value)
        .pretty(true)
        .scroll(es_params.get_scroll())
        .allow_no_indices(true)
        .send()
        .await
}

async fn extract_document_preview(response: Response) -> PaginatedResult<Vec<DocumentPreview>> {
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
        .map(extract_preview)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<DocumentPreview>>();

    PaginatedResult::new_with_opt_id(founded_documents, scroll_id)
}

fn extract_preview(value: &Value) -> Result<DocumentPreview, serde_json::Error> {
    let source_value = &value[&"_source"];
    match DocumentPreview::deserialize(source_value) {
        Ok(docs) => Ok(DocumentPreview::from(docs)),
        Err(err) => {
            log::error!("Failed while deserialize doc: {}", err);
            Err(err)
        }
    }
}

pub(crate) async fn parse_search_result(response: Response) -> PaginatedResult<Vec<Document>> {
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
        .map(extract_highlight)
        .map(Result::ok)
        .filter(Option::is_some)
        .flatten()
        .collect::<Vec<Document>>();

    PaginatedResult::new_with_opt_id(founded_documents, scroll_id)
}

fn extract_highlight(value: &Value) -> Result<Document, serde_json::Error> {
    let source_value = value[&"_source"].to_owned();
    let mut document = Document::deserialize(source_value)?;
    let highlight_value = value[&"highlight"].to_owned();
    let highlight_entity = HighlightEntity::deserialize(highlight_value).ok();
    document.append_highlight(highlight_entity);
    Ok(document)
}

pub(crate) async fn extract_document(response: Response) -> Result<Document, WebError> {
    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    match Document::deserialize(document_json) {
        Err(err) => Err(WebError::from(err)),
        Ok(mut document) => {
            document.exclude_tokens();
            Ok(document)
        }
    }
}

pub(crate) fn build_match_all_query(s_params: &SearchParams) -> Value {
    let (doc_size_from, doc_size_to) = s_params.get_doc_size();
    let (doc_cr_from, _) = s_params.get_doc_dates();
    let location = s_params.get_folders(false);
    let query = s_params.get_query();

    let common_filter = CommonFilter::new()
        .with_date::<FilterRange, CreatedAtDateQuery>("created_at", doc_cr_from, "")
        .with_range::<FilterRange>("file_size", doc_size_from, doc_size_to)
        .with_match::<FilterMatch>("location", location.as_str())
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

pub(crate) fn build_search_query(s_params: &SearchParams) -> Value {
    let (doc_size_from, doc_size_to) = s_params.get_doc_size();
    let (doc_cr_from, doc_cr_to) = s_params.get_doc_dates();
    let doc_ext = s_params.get_extension();
    let doc_type = s_params.get_type();
    let query = s_params.get_query();

    let common_filter = CommonFilter::new()
        .with_date::<FilterRange, CreateDateQuery>("document_created", doc_cr_from, doc_cr_to)
        .with_range::<FilterRange>("document_size", doc_size_from, doc_size_to)
        .with_term::<FilterTerm>("document_extension", doc_ext)
        .with_term::<FilterTerm>("document_type", doc_type)
        .build();

    let match_query = MultiMatchQuery::new(query);
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

    exclude_content_vector(&mut query_json_object);
    query_json_object
}

pub(crate) fn build_search_similar_query(s_params: &SearchParams) -> Value {
    let fields = vec!["content".to_string(), "document_ssdeep".to_string()];
    let ssdeep_hash = s_params.get_query();
    let similar_query = SimilarQuery::new(ssdeep_hash.into(), fields);
    json!({ "query": similar_query })
}

fn exclude_content_vector(es_query: &mut Value) {
    let cont_vector = Some(vec!["content_vector".to_string()]);
    let exclude_fields = ExcludeFields::new(cont_vector);
    let exclude_value = serde_json::to_value(exclude_fields).unwrap();
    es_query[&"_source"] = exclude_value;
}

pub(crate) fn extract_folder_stats(value: &Value) -> Result<Folder, WebError> {
    let indices = &value[&"indices"];
    let folder_id = indices.as_object().unwrap().keys().next().unwrap();

    let index_value = &indices[folder_id.as_str()];
    let health = &index_value[&"health"].as_str().unwrap();
    let status = &index_value[&"status"].as_str().unwrap();
    let uuid = &index_value[&"uuid"].as_str().unwrap();

    let primaries = &value[&"_all"][&"primaries"];
    let docs_count = &primaries[&"docs"][&"count"].as_i64().unwrap();
    let docs_deleted = &primaries[&"docs"][&"deleted"].as_i64().unwrap();
    let store_size = &primaries[&"store"][&"size_in_bytes"].as_i64().unwrap();
    let pri_store_size = &primaries[&"store"][&"total_data_set_size_in_bytes"]
        .as_i64()
        .unwrap();

    let folder = Folder::builder()
        .health(health.to_string())
        .status(status.to_string())
        .index(folder_id.to_owned())
        .uuid(uuid.to_string())
        .docs_count(Some(docs_count.to_string()))
        .docs_deleted(Some(docs_deleted.to_string()))
        .store_size(Some(store_size.to_string()))
        .pri_store_size(Some(pri_store_size.to_string()))
        .pri(None)
        .rep(None)
        .build()
        .map_err(|err| WebError::GetFolder(err.to_string()))?;

    Ok(folder)
}

pub(crate) fn create_folder_schema(is_preview: bool) -> Value {
    if is_preview {
        let schema = DocumentPreviewSchema::default();
        return serde_json::to_value(schema).unwrap();
    }
    let schema = DocumentSchema::default();
    serde_json::to_value(schema).unwrap()
}
