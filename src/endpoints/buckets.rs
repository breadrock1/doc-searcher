use crate::endpoints::ContextData;
use crate::errors::WebResponse;
use crate::wrappers::bucket::{Bucket, BucketForm};

use actix_web::{delete, get, post, web, HttpResponse};

#[utoipa::path(
    post,
    path = "/searcher/buckets",
    tag = "Get all available buckets",
    responses(
        (status = 200, description = "Successful", body = [Bucket]),
        (status = 400, description = "Failed while getting all buckets", body = ErrorResponse),
    )
)]
#[get("/buckets")]
async fn all_buckets(cxt: ContextData) -> WebResponse<web::Json<Vec<Bucket>>> {
    let client = cxt.get_ref();
    client.get_all_buckets().await
}

#[utoipa::path(
    post,
    path = "/searcher/bucket/new",
    tag = "Create new bucket from BucketForm",
    request_body = BucketForm,
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 400, description = "Failed while creating", body = ErrorResponse),
    )
)]
#[post("/bucket/new")]
async fn new_bucket(cxt: ContextData, form: web::Json<BucketForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let bucket_form = form.0;
    client.create_bucket(&bucket_form).await
}

#[utoipa::path(
    post,
    path = "/searcher/bucket/default",
    tag = "Create default bucket (common for all documents)",
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 400, description = "Failed while deleting", body = ErrorResponse),
    )
)]
#[post("/bucket/default")]
async fn default_bucket(cxt: ContextData) -> HttpResponse {
    let client = cxt.get_ref();
    let bucket_form = BucketForm::default();
    client.create_bucket(&bucket_form).await
}

#[utoipa::path(
    post,
    path = "/searcher/bucket/{bucket_name}",
    tag = "Delete bucket",
    params(
        ("bucket_name" = &str, description = "Passed bucket name to delete")
    ),
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 400, description = "Failed while deleting", body = ErrorResponse),
    )
)]
#[delete("/bucket/{bucket_name}")]
async fn delete_bucket(cxt: ContextData, path: web::Path<String>) -> HttpResponse {
    let client = cxt.get_ref();
    let bucket_name = path.to_string();
    client.delete_bucket(bucket_name.as_str()).await
}

#[utoipa::path(
    get,
    path = "/searcher/bucket/{bucket_name}",
    tag = "Get bucket by name",
    params(
        ("bucket_name" = &str, description = "Passed bucket name to get")
    ),
    responses(
        (status = 200, description = "Successful", body = Bucket),
        (status = 400, description = "Failed while getting bucket", body = ErrorResponse),
    )
)]
#[get("/bucket/{bucket_name}")]
async fn get_bucket(cxt: ContextData, path: web::Path<String>) -> WebResponse<web::Json<Bucket>> {
    let client = cxt.get_ref();
    let bucket_name = format!("/{}/_stats", path);
    client.get_bucket(bucket_name.as_str()).await
}

#[cfg(test)]
mod buckets_endpoints {
    use crate::service::own_engine::context::OtherContext;
    use crate::service::ServiceClient;
    use crate::wrappers::bucket::BucketForm;

    use actix_web::test;

    #[test]
    async fn test_create_bucket() {
        let bucket_form = BucketForm::new("test_bucket");
        let other_context = OtherContext::_new("test".to_string());
        let response = other_context.create_bucket(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_delete_bucket() {
        let bucket_form = BucketForm::new("test_bucket");
        let other_context = OtherContext::_new("test".to_string());
        let response = other_context.create_bucket(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let response = other_context.delete_bucket("test_bucket").await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_get_buckets() {
        let bucket_form = BucketForm::new("test_bucket");
        let other_context = OtherContext::_new("test".to_string());
        let response = other_context.create_bucket(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let response = other_context.get_all_buckets().await;
        let buckets_size = response.unwrap().0.len();
        assert_eq!(buckets_size, 1);
    }

    #[test]
    async fn test_get_bucket_by_id() {
        let bucket_form = BucketForm::new("test_bucket");
        let other_context = OtherContext::_new("test".to_string());
        let response = other_context.create_bucket(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let get_bucket_result = other_context.get_bucket("test_bucket").await;
        let bucket_uuid = &get_bucket_result.unwrap().uuid;
        assert_eq!(bucket_uuid.as_str(), "test_bucket");
    }
}
