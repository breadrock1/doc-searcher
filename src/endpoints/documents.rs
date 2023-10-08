use crate::context::SearchContext;
use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::wrappers::Document;

use actix_web::{delete, get, post, put, web, HttpResponse, ResponseError};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::Method;
use elasticsearch::BulkParts;
use serde::Deserialize;
use serde_json::{json, Value};

#[put("/document/update")]
async fn update_document(cxt: web::Data<SearchContext>, form: web::Json<Document>) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
    let bucket_name = &form.bucket_uuid;
    let document_id = &form.document_md5_hash;
    let document_ref = &form.0;
    let document_json = deserialize_document(document_ref);
    if document_json.is_err() {
        let err = document_json.err().unwrap();
        let web_err = WebError::UpdateDocument(err.to_string());
        return web_err.error_response();
    }

    let document_json = document_json.unwrap();
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
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::UpdateDocument(err.to_string());
            web_err.error_response()
        }
    }
}

#[post("/document/new")]
async fn new_document(cxt: web::Data<SearchContext>, form: web::Json<Document>) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
    let bucket_name = &form.bucket_uuid;
    let document_id = &form.document_md5_hash;
    let document_ref = &form.0;
    let to_value_result = serde_json::to_value(document_ref);
    if to_value_result.is_err() {
        let err = to_value_result.err().unwrap();
        let web_err = WebError::DocumentSerializing(err.to_string());
        return web_err.error_response();
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
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::CreateDocument(err.to_string());
            web_err.error_response()
        }
    }
}

#[delete("/document/{bucket_name}/{document_id}")]
async fn delete_document(
    cxt: web::Data<SearchContext>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let elastic = cxt.get_cxt().read().await;
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
        Ok(_) => SuccessfulResponse::ok_response("Ok"),
        Err(err) => {
            let web_err = WebError::DeleteDocument(err.to_string());
            web_err.error_response()
        }
    }
}

#[get("/document/{bucket_name}/{document_id}")]
async fn get_document(
    cxt: web::Data<SearchContext>,
    path: web::Path<(String, String)>,
) -> WebResponse<web::Json<Document>> {
    let elastic = cxt.get_cxt().read().await;
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
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        return Err(WebError::from(err));
    }

    let response = response_result.unwrap();
    let common_object = response.json::<Value>().await.unwrap();
    let document_json = &common_object[&"_source"];
    match Document::deserialize(document_json) {
        Ok(document) => Ok(web::Json(document)),
        Err(err) => Err(WebError::GetDocument(err.to_string())),
    }
}

fn deserialize_document(document_ref: &Document) -> Result<Value, WebError> {
    match serde_json::to_value(document_ref) {
        Ok(value) => Ok(value),
        Err(err) => Err(WebError::DocumentSerializing(err.to_string())),
    }
}

#[cfg(test)]
mod documents_endpoints {
    use crate::context::SearchContext;
    use crate::errors::{ErrorResponse, SuccessfulResponse};
    use crate::es_client::{build_elastic, build_service, init_service_parameters};
    use crate::wrappers::Document;

    use actix_web::test::TestRequest;
    use actix_web::{test, web, App};
    use serde_json::json;

    #[test]
    async fn build_application() {
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
        let test_bucket_name = "test_bucket";
        let test_document_name = "test_document";
        let test_document_path = "/tmp/dir/";
        let test_document_id = "79054025255fb1a26e4bc422aef54eb4";

        // Create new document with name: "test_document"
        let create_document_resp = TestRequest::post()
            .uri("/searcher/document/new")
            .set_json(&json!({
                "bucket_uuid": test_bucket_name,
                "bucket_path": "/tmp/test_document",
                "document_name": test_document_name,
                "document_path": test_document_path,
                "document_size": 1024,
                "document_type": "document",
                "document_extension": ".docx",
                "document_permissions": 777,
                "document_created": "2023-09-15T00:00:00Z",
                "document_modified": "2023-09-15T00:00:00Z",
                "document_md5_hash": test_document_id,
                "document_ssdeep_hash": "3a:34gh5",
                "entity_data": "Using skip_serializing does not skip deserializing the field.",
                "entity_keywords": ["document", "report"]
            }))
            .send_request(&test_app)
            .await;

        let new_document: SuccessfulResponse = test::read_body_json(create_document_resp).await;
        assert_eq!(new_document.code, 200);

        // Get document request by document name
        let get_document_resp = TestRequest::get()
            .uri(&format!(
                "/searcher/document/{}/{}",
                test_bucket_name, test_document_id
            ))
            .send_request(&test_app)
            .await;

        let get_document: Document = test::read_body_json(get_document_resp).await;
        assert_eq!(get_document.document_md5_hash, test_document_id);

        // Get document request by document name after updating
        let update_document_resp = TestRequest::put()
            .uri("/searcher/document/update")
            .set_json(&json!({
                "bucket_uuid": test_bucket_name,
                "bucket_path": "/tmp/test_document",
                "document_name": test_document_name,
                "document_path": "./",
                "document_size": 1024,
                "document_type": "document",
                "document_extension": ".docx",
                "document_permissions": 777,
                "document_created": "2023-09-15T00:00:00Z",
                "document_modified": "2023-09-15T00:00:00Z",
                "document_md5_hash": test_document_id,
                "document_ssdeep_hash": "3a:34gh5",
                "entity_data": "Using skip_serializing does not skip deserializing the field.",
                "entity_keywords": ["document", "report"]
            }))
            .send_request(&test_app)
            .await;

        let update_document: SuccessfulResponse = test::read_body_json(update_document_resp).await;
        assert_eq!(update_document.code, 200);

        let get_updated_document_resp = TestRequest::get()
            .uri(&format!(
                "/searcher/document/{}/{}",
                test_bucket_name, test_document_id
            ))
            .send_request(&test_app)
            .await;

        let get_document: Document = test::read_body_json(get_updated_document_resp).await;
        assert_eq!(get_document.document_path, "./");

        // Delete document by index
        let delete_document_resp = TestRequest::delete()
            .uri(&format!(
                "/searcher/document/{}/{}",
                test_bucket_name, test_document_id
            ))
            .send_request(&test_app)
            .await;

        let delete_document: SuccessfulResponse = test::read_body_json(delete_document_resp).await;
        assert_eq!(delete_document.code, 200);

        // Get document by index -> get error message
        let get_document_err_resp = TestRequest::get()
            .uri(&format!(
                "/searcher/document/{}/{}",
                test_bucket_name, "lsdfnbsikdjfsidg"
            ))
            .send_request(&test_app)
            .await;

        let get_document_err: ErrorResponse = test::read_body_json(get_document_err_resp).await;
        assert_eq!(get_document_err.code, 400);
    }
}
