use crate::context::SearchContext;
use crate::errors::{WebError, WebResponse};
use crate::wrappers::{Bucket, BucketForm, StatusResult};

use actix_web::{delete, get, post, web};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use elasticsearch::IndexParts;
use serde_json::{json, Value};

#[get("/buckets")]
async fn all_buckets(cxt: web::Data<SearchContext>) -> WebResponse<web::Json<Vec<Bucket>>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let body = b"";
    let response = elastic
        .send(
            Method::Get,
            "/_cat/indices?format=json",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await?;

    match response.json::<Vec<Bucket>>().await {
        Ok(json_buckets) => Ok(web::Json(json_buckets)),
        Err(err) => {
            println!("{:?}", err);
            Err(WebError::SomeError(err.to_string()))
        }
    }
}

#[post("/bucket/new")]
async fn new_bucket(
    cxt: web::Data<SearchContext>,
    form: web::Form<BucketForm>,
) -> WebResponse<web::Json<StatusResult>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let bucket_name = form.0.to_string();
    let digest = md5::compute(bucket_name.as_str());
    let id_str = format!("{:x}", digest);
    let response = elastic
        .index(IndexParts::IndexId(bucket_name.as_str(), id_str.as_str()))
        .body(json!({
            bucket_name.as_str(): {
                "_source": { "enabled": false },
                "properties": {
                    "bucket_uuid": { "type": "string" },
                    "bucket_path": { "type": "string" },
                    "document_name": { "type": "string" },
                    "document_path": { "type": "string" },
                    "document_size": { "type": "integer" },
                    "document_type": { "type": "string" },
                    "document_extension": { "type": "string" },
                    "document_permissions": { "type": "integer" },
                    "document_created": { "type": "date" },
                    "document_modified": { "type": "date" },
                    "document_md5_hash": { "type": "string" },
                    "document_ssdeep_hash": { "type": "string" },
                    "entity_keywords": [],
                }
            }
        }))
        .send()
        .await?;

    let result = response.status_code().as_u16();
    Ok(web::Json(StatusResult::new(result)))
}

#[delete("/bucket/delete")]
async fn delete_bucket(
    cxt: web::Data<SearchContext>,
    form: web::Form<BucketForm>,
) -> WebResponse<web::Json<StatusResult>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let bucket_name = format!("/{}", form.0.to_string());
    let body = b"";
    let response = elastic
        .send(
            Method::Delete,
            bucket_name.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await?;

    let result = response.status_code().as_u16();
    Ok(web::Json(StatusResult::new(result)))
}

#[get("/bucket/{bucket_name}")]
async fn get_bucket(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
) -> WebResponse<web::Json<Value>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let bucket_name = format!("/{}", path.to_string());
    let body = b"";
    let response = elastic
        .send(
            Method::Get,
            bucket_name.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(body.as_ref()),
            None,
        )
        .await?;

    match response.json::<Value>().await {
        Ok(cluster_info) => Ok(web::Json(cluster_info)),
        Err(err) => {
            println!("{:?}", err);
            Err(WebError::SomeError(err.to_string()))
        }
    }
}
