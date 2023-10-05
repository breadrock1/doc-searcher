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
    let elastic = cxt.get_cxt().lock().unwrap();
    let bucket_name = &form.bucket_uuid;
    let document_id = &form.document_md5_hash;
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let document_ref = &form.0;
    let document_json = serde_json::to_value(document_ref);
    if document_json.is_err() {
        let err_msg = document_json.err().unwrap().to_string();
        return Err(WebError::SomeError(err_msg));
    }

    let response = elastic
        .send(
            Method::Put,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(document_json.unwrap().to_string().as_bytes()),
            None,
        )
        .await?;

    let result = response.status_code().as_u16();
    Ok(web::Json(StatusResult::new(result)))
}

#[post("/document/new")]
async fn new_document(
    cxt: web::Data<SearchContext>,
    form: web::Json<Document>,
) -> WebResponse<web::Json<StatusResult>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let bucket_name = &form.bucket_uuid;
    let document_id = &form.document_md5_hash;
    let document_ref = &form.0;
    let document_json = serde_json::to_value(document_ref);
    if document_json.is_err() {
        let err_msg = document_json.err().unwrap().to_string();
        return Err(WebError::SomeError(err_msg));
    }

    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(2);
    body.push(json!({"index": { "_id": document_id.as_str() }}).into());
    body.push(document_json.unwrap().into());

    let response = elastic
        .bulk(BulkParts::Index(bucket_name.as_str()))
        .body(body)
        .send()
        .await
        .unwrap();

    let result = response.status_code().as_u16();
    Ok(web::Json(StatusResult::new(result)))
}

#[delete("/document/{bucket_name}/{document_id}")]
async fn delete_document(
    cxt: web::Data<SearchContext>,
    path: web::Path<(String, String)>,
) -> WebResponse<web::Json<StatusResult>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let (bucket_name, document_id) = path.as_ref();
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let response = elastic
        .send(
            Method::Delete,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await?;

    let result = response.status_code().as_u16();
    Ok(web::Json(StatusResult::new(result)))
}

#[get("/document/{bucket_name}/{document_id}")]
async fn get_document(
    cxt: web::Data<SearchContext>,
    path: web::Path<(String, String)>,
) -> WebResponse<web::Json<Document>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let (bucket_name, document_id) = path.as_ref();
    let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
    let response = elastic
        .send(
            Method::Get,
            s_path.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await?;

    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    match Document::deserialize(document_json) {
        Ok(document) => Ok(web::Json(document)),
        Err(err) => {
            println!("{:?}", err);
            Err(WebError::SomeError(err.to_string()))
        }
    }
}
