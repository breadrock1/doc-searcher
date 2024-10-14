use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::searcher::models::{Paginated, SearchParams};
use crate::searcher::SearcherTrait;
use crate::storage::forms::DocumentType;
use crate::storage::models::{Document, DocumentPreview, DocumentVectors};
use crate::storage::DocumentsTrait;

use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::response::Response;
use elasticsearch::http::Method;
use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::Value;
use std::collections::HashMap;

pub async fn send_elrequest(
    elastic: &Elasticsearch,
    method: Method,
    body: Option<&[u8]>,
    target_url: &str,
) -> Result<Response, WebError> {
    let response = elastic
        .send(
            method,
            target_url,
            HeaderMap::new(),
            Option::<&Value>::None,
            body,
            None,
        )
        .await
        .map_err(WebError::from)?;

    match response.status_code().is_success() {
        false => Err(extract_exception(response).await),
        true => Ok(response),
    }
}

pub async fn parse_elastic_response(response: Response) -> WebResult<Successful> {
    if !response.status_code().is_success() {
        return Err(extract_exception(response).await);
    }

    let txt = response.text().await.unwrap();
    tracing::warn!("{}", txt.as_str());
    Ok(Successful::success(txt.as_str()))
}

pub async fn extract_exception(response: Response) -> WebError {
    let test = response.json::<Value>().await.unwrap();
    let text = serde_json::to_string_pretty(&test).unwrap();
    // let text = response.text().await.unwrap();
    let entity = WebErrorEntity::new(text);
    WebError::UnknownError(entity)
    // let exception_res = response.exception().await;
    // if exception_res.is_err() {
    //     let err = exception_res.err().unwrap();
    //     let entity = WebErrorEntity::new(err.to_string());
    //     return WebError::UnknownError(entity);
    // }
    //
    // match exception_res.unwrap() {
    //     Some(exception) => WebError::from(exception),
    //     None => {
    //         let entity = WebErrorEntity::new("Unknown error".to_string());
    //         WebError::UnknownError(entity)
    //     },
    // }
}

pub fn to_unified_pag(
    mut paginated: Paginated<Vec<Document>>,
    doc_type: &DocumentType,
) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.get_scroll_id().cloned();
    let converted = paginated
        .get_founded_mut()
        .iter()
        .flat_map(|doc| doc_type.to_value(doc))
        .collect::<Vec<Value>>();

    Paginated::new_with_opt_id(converted, scroll_id)
}

pub fn vec_to_value(mut paginated: Paginated<Vec<DocumentVectors>>) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.get_scroll_id().cloned();
    let converted = paginated
        .get_founded_mut()
        .iter()
        .flat_map(serde_json::to_value)
        .collect::<Vec<Value>>();

    Paginated::new_with_opt_id(converted, scroll_id)
}

pub fn vec_to_grouped_value(paginated: Paginated<Vec<DocumentVectors>>) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.get_scroll_id().cloned();
    let converted = group_document_chunks(paginated.get_founded());
    let values = serde_json::to_value(converted).unwrap();
    Paginated::new_with_opt_id(vec![values], scroll_id)
}

fn group_document_chunks(documents: &[DocumentVectors]) -> HashMap<String, Vec<DocumentVectors>> {
    let mut grouped_documents: HashMap<String, Vec<DocumentVectors>> = HashMap::new();
    documents.iter().for_each(|doc| {
        grouped_documents
            .entry(doc.get_doc_id().to_string())
            .or_default()
            .push(doc.to_owned())
    });

    grouped_documents
}

pub async fn search<T>(
    elastic: &Elasticsearch,
    s_params: &SearchParams,
    indexes: &[&str],
) -> Result<Paginated<Vec<T>>, WebError>
where
    T: DocumentsTrait + SearcherTrait<T>,
{
    let body_value = T::build_query(s_params).await;
    let response = send_search_request(elastic, s_params, &body_value, indexes).await?;
    if !response.status_code().is_success() {
        let msg = response.json::<Value>().await.unwrap();
        let msg = serde_json::to_string_pretty(&msg).unwrap();
        let entity = WebErrorEntity::new(msg);
        return Err(WebError::SearchError(entity));
    }
    Ok(extract_elastic_response(response).await)
}

pub async fn search_all<T>(
    elastic: &Elasticsearch,
    s_params: &SearchParams,
    indexes: &[&str],
) -> Result<Paginated<Vec<T>>, WebError>
where
    T: DocumentsTrait + SearcherTrait<T>,
{
    let body_value = DocumentPreview::build_query(s_params).await;
    let response = send_search_request(elastic, s_params, &body_value, indexes).await?;
    if !response.status_code().is_success() {
        let msg = response.json::<Value>().await.unwrap();
        let msg = serde_json::to_string_pretty(&msg).unwrap();
        let entity = WebErrorEntity::new(msg.to_string());
        return Err(WebError::SearchError(entity));
    }
    Ok(extract_elastic_response::<T>(response).await)
}

pub async fn send_search_request(
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
        .scroll(es_params.scroll_lifetime())
        .allow_no_indices(true)
        .send()
        .await
}

pub async fn extract_elastic_response<T>(response: Response) -> Paginated<Vec<T>>
where
    T: DocumentsTrait + SearcherTrait<T>,
{
    let common_object = response.json::<Value>().await.unwrap();
    let document_json = &common_object[&"hits"][&"hits"];
    let scroll_id = common_object[&"_scroll_id"]
        .as_str()
        .map_or_else(|| None, |x| Some(x.to_string()));

    let own_document = document_json.to_owned();
    let default_vec: Vec<Value> = Vec::default();
    let json_array = own_document.as_array().unwrap_or(&default_vec);

    let mut extracted_values: Vec<Result<T, WebError>> = Vec::default();
    for doc_value in json_array.iter() {
        let extracted = T::extract_from_response(doc_value).await;
        extracted_values.push(extracted);
    }

    let founded_documents = extracted_values.into_iter().flatten().collect::<Vec<T>>();

    Paginated::new_with_opt_id(founded_documents, scroll_id)
}
