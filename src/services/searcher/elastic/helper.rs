use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::forms::documents::DocumentsTrait;
use crate::forms::documents::document::Document;
use crate::forms::documents::vector::DocumentVectors;
use crate::forms::documents::forms::DocumentType;
use crate::forms::pagination::pagination::Paginated;
use crate::services::searcher::elastic::context::ContextOptions;

use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::response::Response;
use elasticsearch::http::Method;
use elasticsearch::Elasticsearch;
use serde_json::{json, Value};
use std::collections::HashMap;

pub(crate) async fn send_llm_request(cxt_opts: &ContextOptions, query: &str) -> Vec<f64> {
    let target_url = format!("{}/embed", cxt_opts.get_llm_addr());
    let response_result = reqwest::Client::new()
        .post(target_url)
        .json(&json!({
            "inputs": query,
            "normalize": false,
            "truncate": false
        }))
        .send()
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        log::error!("Failed to get embeddings: {}", err);
        return Vec::default();
    }

    let response = response_result
        .unwrap()
        .json::<Vec<Vec<f64>>>()
        .await
        .unwrap();

    let data1 = response.first().unwrap();
    data1.to_vec()
}

pub(crate) async fn send_elrequest(
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

pub(crate) async fn parse_elastic_response(response: Response) -> WebResult<Successful> {
    if !response.status_code().is_success() {
        return Err(extract_exception(response).await);
    }

    let txt = response.text().await.unwrap();
    log::warn!("{}", txt.as_str());
    Ok(Successful::success(txt.as_str()))
}

pub(crate) async fn extract_exception(response: Response) -> WebError {
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

pub(crate) fn to_unified_pag(mut paginated: Paginated<Vec<Document>>, doc_type: &DocumentType) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.get_scroll_id().cloned();
    let converted = paginated
        .get_founded_mut()
        .iter()
        .map(|doc| doc_type.to_value(doc))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<Value>>();

    Paginated::new_with_opt_id(converted, scroll_id)
}

pub(crate) fn vec_to_value(mut paginated: Paginated<Vec<DocumentVectors>>) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.get_scroll_id().cloned();
    let converted = paginated
        .get_founded_mut()
        .iter()
        .map(|doc| serde_json::to_value(doc))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<Value>>();

    Paginated::new_with_opt_id(converted, scroll_id)
}

pub(crate) fn vec_to_grouped_value(paginated: Paginated<Vec<DocumentVectors>>) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.get_scroll_id().cloned();
    let converted = group_document_chunks(paginated.get_founded());
    let values = serde_json::to_value(converted).unwrap();
    Paginated::new_with_opt_id(vec![values], scroll_id)
}

fn group_document_chunks(documents: &[DocumentVectors]) -> HashMap<String, Vec<DocumentVectors>> {
    let mut grouped_documents: HashMap<String, Vec<DocumentVectors>> = HashMap::new();
    documents
        .iter()
        .for_each(|doc| {
            grouped_documents
                .entry(doc.get_doc_id().to_string())
                .or_default()
                .push(doc.to_owned())
        });

    grouped_documents
}
