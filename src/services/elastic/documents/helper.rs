use crate::errors::{WebError, WebResult};
use crate::forms::documents::document::Document;
use crate::forms::preview::DocumentPreview;
use crate::services::elastic::helper;

use elasticsearch::http::request::JsonBody;
use elasticsearch::http::response::Response;
use elasticsearch::{BulkParts, CountParts, Elasticsearch};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::RwLockReadGuard;

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

pub(crate) async fn store_document(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    doc_form: &Document,
    folder_id: &str,
) -> WebResult {
    let to_value_result = serde_json::to_value(doc_form);
    let document_json = to_value_result.unwrap();
    let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

    body.push(json!({"index": { "_id": doc_form.get_doc_id() }}).into());
    body.push(document_json.into());

    let response = elastic
        .bulk(BulkParts::Index(folder_id))
        .body(body)
        .send()
        .await
        .map_err(WebError::from)?;

    helper::parse_elastic_response(response).await
}

pub(crate) async fn store_preview(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    doc_form: &DocumentPreview,
    folder_id: &str,
) -> WebResult {
    let to_value_result = serde_json::to_value(doc_form);
    let document_json = to_value_result.unwrap();
    let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

    body.push(json!({"index": { "_id": doc_form.get_id() }}).into());
    body.push(document_json.into());

    let response = elastic
        .bulk(BulkParts::Index(folder_id))
        .body(body)
        .send()
        .await
        .map_err(WebError::from)?;

    helper::parse_elastic_response(response).await
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

    let result = response.json::<Value>().await.map_or(false, |value| {
        let count = value["count"].as_i64().unwrap_or(0);
        count > 0
    });

    Ok(result)
}
