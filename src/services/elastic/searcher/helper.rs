use crate::errors::{PaginateResponse, WebError};
use crate::forms::documents::document::{Document, HighlightEntity};
use crate::forms::pagination::Paginated;
use crate::forms::preview::DocumentPreview;
use crate::forms::s_params::SearchParams;

use actix_web::web;
use elasticsearch::http::response::Response;
use elasticsearch::{Elasticsearch, SearchParts};
use elquery::exclude_fields::ExcludeFields;
use elquery::filter_query::{
    CommonFilter, CreateDateQuery, CreatedAtDateQuery, FilterMatch, FilterRange, FilterTerm,
};
use elquery::highlight_query::HighlightOrder;
use elquery::search_query::MultiMatchQuery;
use elquery::similar_query::SimilarQuery;
use serde::Deserialize;
use serde_json::{json, Value};

pub(crate) fn build_semantic_query(s_params: &SearchParams) -> Value {
    let _query = s_params.get_query();
    let query_vector: Vec<f64> = Vec::default();
    json!({
        "knn": {
            "field": "text_vector.vector",
            "query_vector": query_vector,
            "k": 1,
            "num_candidates": 10,
            "inner_hits": {
                "_source": false,
                "size": 1,
                "fields": [
                    "text_vector.text_chunk"
                ]
            }
        }
    })
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

pub(crate) async fn search_documents_preview(
    elastic: &Elasticsearch,
    es_params: &SearchParams,
    body_value: &Value,
    indexes: &[&str],
) -> PaginateResponse<Vec<DocumentPreview>> {
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
) -> PaginateResponse<Vec<Document>> {
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

async fn extract_document_preview(response: Response) -> Paginated<Vec<DocumentPreview>> {
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

    Paginated::new_with_opt_id(founded_documents, scroll_id)
}

fn extract_preview(value: &Value) -> Result<DocumentPreview, serde_json::Error> {
    let source_value = &value[&"_source"];
    match DocumentPreview::deserialize(source_value) {
        Ok(docs) => Ok(docs),
        Err(err) => {
            log::error!("Failed while deserialize doc: {}", err);
            Err(err)
        }
    }
}

pub(crate) async fn parse_search_result(response: Response) -> Paginated<Vec<Document>> {
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

    Paginated::new_with_opt_id(founded_documents, scroll_id)
}

fn extract_highlight(value: &Value) -> Result<Document, serde_json::Error> {
    let source_value = value[&"_source"].to_owned();
    let mut document = Document::deserialize(source_value)?;
    let highlight_value = value[&"highlight"].to_owned();
    let highlight_entity = HighlightEntity::deserialize(highlight_value).ok();
    document.append_highlight(highlight_entity);
    Ok(document)
}
