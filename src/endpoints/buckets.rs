use crate::endpoints::ContextData;
use crate::errors::WebResponse;
use crate::searcher::service_client::ServiceClient;
use crate::wrappers::bucket::{Bucket, BucketForm};

use actix_web::{delete, get, post, web, HttpResponse};

#[get("/buckets")]
async fn all_buckets(cxt: ContextData) -> WebResponse<web::Json<Vec<Bucket>>> {
    let client = cxt.get_ref();
    client.get_all_buckets().await
}

#[post("/bucket/new")]
async fn new_bucket(
    cxt: web::Data<&dyn ServiceClient>,
    form: web::Json<BucketForm>,
) -> HttpResponse {
    let client = cxt.get_ref();
    let bucket_form = form.0;
    client.create_bucket(&bucket_form).await
}

#[delete("/bucket/{bucket_name}")]
async fn delete_bucket(cxt: ContextData, path: web::Path<String>) -> HttpResponse {
    let client = cxt.get_ref();
    let bucket_name = path.to_string();
    client.delete_bucket(bucket_name.as_str()).await
}

#[get("/bucket/{bucket_name}")]
async fn get_bucket(cxt: ContextData, path: web::Path<String>) -> WebResponse<web::Json<Bucket>> {
    let client = cxt.get_ref();
    let bucket_name = format!("/{}/_stats", path);
    client.get_bucket(bucket_name.as_str()).await
}

#[cfg(test)]
mod buckets_endpoints {
    use crate::errors::{ErrorResponse, SuccessfulResponse};
    use crate::searcher::elastic::build_elastic_client;
    use crate::searcher::elastic::context::ElasticContext;
    use crate::service::{build_service, init_service_parameters};
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

        let elastic = build_elastic_client(es_host, es_user, es_passwd).unwrap();
        let cxt = ElasticContext::_new(elastic);
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

        let elastic = build_elastic_client(es_host, es_user, es_passwd).unwrap();
        let cxt = ElasticContext::_new(elastic);
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
