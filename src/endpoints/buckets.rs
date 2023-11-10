use crate::context::SearchContext;
use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::wrappers::bucket::{Bucket, BucketForm};

use actix_web::{delete, get, post, web, HttpResponse, ResponseError};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use elasticsearch::IndexParts;
use serde_json::{json, Value};

#[get("/buckets")]
async fn all_buckets(cxt: web::Data<SearchContext>) -> WebResponse<web::Json<Vec<Bucket>>> {
    let elastic = cxt.get_cxt().read().await;
    let response_result = elastic
        .send(
            Method::Get,
            "/_cat/indices?format=json",
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        return Err(WebError::from(err));
    }

    let response = response_result.unwrap();
    match response.json::<Vec<Bucket>>().await {
        Ok(json_buckets) => Ok(web::Json(json_buckets)),
        Err(err) => {
            println!("{:?}", err.to_string().as_str());
            Err(WebError::GetBucket(err.to_string()))
        },
    }
}

#[post("/bucket/new")]
async fn new_bucket(cxt: web::Data<SearchContext>, form: web::Json<BucketForm>) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
    let bucket_name = form.0.to_string();
    let digest = md5::compute(bucket_name.as_str());
    let id_str = format!("{:x}", digest);
    let response_result = elastic
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
                    "entity_data": { "type": "string" },
                    "entity_keywords": [],
                }
            }
        }))
        .send()
        .await;

    match response_result {
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::CreateBucket(err.to_string());
            web_err.error_response()
        }
    }
}

#[delete("/bucket/{bucket_name}")]
async fn delete_bucket(cxt: web::Data<SearchContext>, path: web::Path<String>) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
    let bucket_name = path.to_string();
    let response_result = elastic
        .send(
            Method::Delete,
            bucket_name.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await;

    match response_result {
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::DeleteBucket(err.to_string());
            web_err.error_response()
        }
    }
}

#[get("/bucket/{bucket_name}")]
async fn get_bucket(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
) -> WebResponse<web::Json<Bucket>> {
    let elastic = cxt.get_cxt().read().await;
    let bucket_name = format!("/{}/_stats", path.to_string());
    let response_result = elastic
        .send(
            Method::Get,
            bucket_name.as_str(),
            HeaderMap::new(),
            Option::<&Value>::None,
            Some(b"".as_ref()),
            None,
        )
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        return Err(WebError::from(err));
    }

    let response = response_result.unwrap();
    match response.json::<Value>().await {
        Err(err) => Err(WebError::GetBucket(err.to_string())),
        Ok(value) => match extract_bucket_stats(&value) {
            Ok(data) => Ok(web::Json(data)),
            Err(err) => Err(err),
        },
    }
}

fn extract_bucket_stats(value: &Value) -> Result<Bucket, WebError> {
    let indicies = &value[&"indices"];
    let bucket_id = indicies.as_object();
    if bucket_id.is_none() {
        let msg = String::from("There is no passed bucket name in json.");
        return Err(WebError::BucketParsing(msg));
    }

    let bucket_id = bucket_id.unwrap().keys().next().unwrap();
    let bucket = &indicies[bucket_id.as_str()];
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

    Ok(Bucket::new(
        health.to_string(),
        status.to_string(),
        bucket_id.to_string(),
        uuid.to_string(),
        docs_count.to_string(),
        docs_deleted.to_string(),
        store_size.to_string(),
        pri_store_size.to_string(),
        None,
        None,
    ))
}

#[cfg(test)]
mod buckets_endpoints {
    use crate::context::SearchContext;
    use crate::errors::{ErrorResponse, SuccessfulResponse};
    use crate::es_client::{build_elastic, build_service, init_service_parameters};
    use crate::wrappers::bucket::Bucket;

    use actix_web::test::TestRequest;
    use actix_web::{test, web, App};
    use serde_json::json;

    #[test]
    async fn build_application() {
        let service_parameters = init_service_parameters().unwrap();
        let es_host = service_parameters.es_host();
        let es_user = service_parameters.es_user();
        let es_passwd = service_parameters.es_passwd();

        let elastic = build_elastic(es_host, es_user, es_passwd).unwrap();
        let cxt = SearchContext::_new(elastic);
        let app = App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service());

        let test_app = test::init_service(app).await;
        let test_bucket_name = "test_bucket";

        // Create new bucket with name: "test_bucket"
        let create_bucket_resp = TestRequest::post()
            .uri("/searcher/bucket/new")
            .set_json(&json!({"bucket_name": test_bucket_name}))
            .send_request(&test_app)
            .await;

        let new_bucket: SuccessfulResponse = test::read_body_json(create_bucket_resp).await;
        assert_eq!(new_bucket.code, 200);

        // Get all buckets request
        let get_all_buckets_resp = TestRequest::get()
            .uri("/searcher/buckets")
            .send_request(&test_app)
            .await;

        let get_all_buckets: Vec<Bucket> = test::read_body_json(get_all_buckets_resp).await;
        assert_eq!(get_all_buckets.len(), 1);

        // Get bucket request by bucket name
        let get_bucket_resp = TestRequest::get()
            .uri(&format!("/searcher/bucket/{}", test_bucket_name))
            .send_request(&test_app)
            .await;

        let get_bucket: Bucket = test::read_body_json(get_bucket_resp).await;
        assert_eq!(get_bucket.index, test_bucket_name);

        // Delete bucket by index
        let delete_bucket_resp = TestRequest::delete()
            .uri(&format!("/searcher/bucket/{}", test_bucket_name))
            .send_request(&test_app)
            .await;

        let delete_bucket: SuccessfulResponse = test::read_body_json(delete_bucket_resp).await;
        assert_eq!(delete_bucket.code, 200);

        // Get bucket by index -> get error message
        let get_bucket_err_resp = TestRequest::get()
            .uri(&format!("/searcher/bucket/{}", "lsdfnbsikdjfsidg"))
            .send_request(&test_app)
            .await;

        let get_bucket_err: ErrorResponse = test::read_body_json(get_bucket_err_resp).await;
        assert_eq!(get_bucket_err.code, 400);
    }

    #[test]
    async fn create_buckets_integration_test() {
        let service_parameters = init_service_parameters().unwrap();
        let es_host = service_parameters.es_host();
        let es_user = service_parameters.es_user();
        let es_passwd = service_parameters.es_passwd();
        let service_port = service_parameters.service_port();
        let service_addr = service_parameters.service_address();

        let elastic = build_elastic(es_host, es_user, es_passwd).unwrap();
        let cxt = SearchContext::_new(elastic);
        let app = App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service());

        let test_app = test::init_service(app).await;
        let test_bucket_name = "docs";

        // Create new bucket with name: "test_bucket"
        let create_bucket_resp = TestRequest::post()
            .uri("/searcher/bucket/new")
            .set_json(&json!({"bucket_name": test_bucket_name}))
            .send_request(&test_app)
            .await;

        let new_bucket: SuccessfulResponse = test::read_body_json(create_bucket_resp).await;
        assert_eq!(new_bucket.code, 200);
    }
}
