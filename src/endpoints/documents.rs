use crate::context::SearchContext;
use crate::errors::{WebError, WebResponse};
use crate::wrappers::{Document, StatusResult};

use actix_web::{delete, get, post, put, web};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::Method;
use elasticsearch::BulkParts;
use serde::Deserialize;
use serde_json::{json, Value};

#[put("/document/update")]
async fn update_document(
    cxt: web::Data<SearchContext>,
    form: web::Json<Document>,
) -> WebResponse<web::Json<StatusResult>> {
    let elastic = cxt.get_cxt().blocking_read();
    let bucket_name = &form.bucket_uuid;
    let document_id = &form.document_md5_hash;
    let document_ref = &form.0;
    let to_value_result = serde_json::to_value(document_ref);
    if to_value_result.is_err() {
        let err = to_value_result.err().unwrap();
        return Err(WebError::DocumentSerializingError(err.to_string()));
    }

    let document_json = to_value_result.unwrap();
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let response_result = elastic
        .send(
            Method::Put,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(document_json.to_string().as_bytes()),
            None,
        )
        .await;

    match response_result {
        Err(err) => Err(WebError::UpdateDocumentError(err.to_string())),
        Ok(response) => {
            let result = response.status_code().as_u16();
            Ok(web::Json(StatusResult::new(result)))
        }
    }
}

#[post("/document/new")]
async fn new_document(
    cxt: web::Data<SearchContext>,
    form: web::Json<Document>,
) -> WebResponse<web::Json<StatusResult>> {
    let elastic = cxt.get_cxt().blocking_read();
    let bucket_name = &form.bucket_uuid;
    let document_id = &form.document_md5_hash;
    let document_ref = &form.0;
    let to_value_result = serde_json::to_value(document_ref);
    if to_value_result.is_err() {
        let err = to_value_result.err().unwrap();
        return Err(WebError::DocumentSerializingError(err.to_string()));
    }

    let document_json = to_value_result.unwrap();
    let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);
    body.push(json!({"index": { "_id": document_id.as_str() }}).into());
    body.push(document_json.into());

    let response_result = elastic
        .bulk(BulkParts::Index(bucket_name.as_str()))
        .body(body)
        .send()
        .await;

    match response_result {
        Err(err) => Err(WebError::CreateDocumentError(err.to_string())),
        Ok(response) => {
            let result = response.status_code().as_u16();
            Ok(web::Json(StatusResult::new(result)))
        }
    }
}

#[delete("/document/{bucket_name}/{document_id}")]
async fn delete_document(
    cxt: web::Data<SearchContext>,
    path: web::Path<(String, String)>,
) -> WebResponse<web::Json<StatusResult>> {
    let elastic = cxt.get_cxt().blocking_read();
    let (bucket_name, document_id) = path.as_ref();
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let response_result = elastic
        .send(
            Method::Delete,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await;

    match response_result {
        Err(err) => Err(WebError::DeleteDocumentError(err.to_string())),
        Ok(response) => {
            let result = response.status_code().as_u16();
            Ok(web::Json(StatusResult::new(result)))
        }
    }
}

#[get("/document/{bucket_name}/{document_id}")]
async fn get_document(
    cxt: web::Data<SearchContext>,
    path: web::Path<(String, String)>,
) -> WebResponse<web::Json<Document>> {
    let elastic = cxt.get_cxt().blocking_read();
    let (bucket_name, document_id) = path.as_ref();
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let response_result = elastic
        .send(
            Method::Get,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await?;

    let common_object = response_result.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    match Document::deserialize(document_json) {
        Ok(document) => Ok(web::Json(document)),
        Err(err) => Err(WebError::GetDocumentError(err.to_string())),
    }
}
