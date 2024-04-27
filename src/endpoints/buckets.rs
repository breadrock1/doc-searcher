use crate::endpoints::SearcherData;
use crate::errors::{JsonResponse, PaginateJsonResponse, ErrorResponse, SuccessfulResponse};

use actix_web::{delete, get, post, web, HttpResponse};

use wrappers::bucket::{Bucket, BucketForm};
use wrappers::document::Document;

#[utoipa::path(
    get,
    path = "/bucket/all",
    tag = "Buckets",
    responses(
        (
            status = 200, 
            description = "Successful", 
            body = [Bucket], 
            example = json!(vec![Bucket::default()])
        ),
        (
            status = 400, 
            description = "Failed while getting all buckets", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting buckets".to_string(),
            })
        ),
    )
)]
#[get("/all")]
async fn all_buckets(cxt: SearcherData) -> JsonResponse<Vec<Bucket>> {
    let client = cxt.get_ref();
    client.get_all_buckets().await
}

#[utoipa::path(
    post,
    path = "/bucket/new",
    tag = "Buckets",
    request_body(
        content = BucketForm, 
        example = json!({
            "bucket_name": "test_bucket"
        })
    ),
    responses(
        (
            status = 200, 
            description = "Successful", 
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400, 
            description = "Failed while creating new bucket", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while creating new bucket".to_string(),
            })
        ),
    )
)]
#[post("/new")]
async fn new_bucket(cxt: SearcherData, form: web::Json<BucketForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let bucket_form = form.0;
    client.create_bucket(&bucket_form).await
}

#[utoipa::path(
    post,
    path = "/bucket/default",
    tag = "Buckets",
    responses(
        (
            status = 200, 
            description = "Successful", 
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400, 
            description = "Failed while creating default bucket", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while creating default bucket".to_string(),
            })
        ),
    )
)]
#[post("/default")]
async fn default_bucket(cxt: SearcherData) -> HttpResponse {
    let client = cxt.get_ref();
    let bucket_form = BucketForm::default();
    client.create_bucket(&bucket_form).await
}

#[utoipa::path(
    delete,
    path = "/bucket/{bucket_name}",
    tag = "Buckets",
    params(
        (
            "bucket_name" = &str, 
            description = "Passed bucket name to delete",
            example = "test_bucket",
        )
    ),
    responses(
        (
            status = 200, 
            description = "Successful", 
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Done".to_string(),
            })
        ),
        (
            status = 400, 
            description = "Failed while deleting bucket", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while deleting bucket".to_string(),
            })
        ),
    )
)]
#[delete("/{bucket_name}")]
async fn delete_bucket(cxt: SearcherData, path: web::Path<String>) -> HttpResponse {
    let client = cxt.get_ref();
    let bucket_name = path.to_string();
    client.delete_bucket(bucket_name.as_str()).await
}

#[utoipa::path(
    get,
    path = "/bucket/{bucket_name}",
    tag = "Buckets",
    params(
        (
            "bucket_name" = &str, 
            description = "Passed bucket name to get",
            example = "test_bucket",
        )
    ),
    responses(
        (
            status = 200, 
            description = "Successful", 
            body = Bucket,
            example = json!(Bucket::default())
        ),
        (
            status = 400, 
            description = "Failed while getting bucket by name", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting bucket by name".to_string(),
            })
        ),
    )
)]
#[get("/{bucket_name}")]
async fn get_bucket(cxt: SearcherData, path: web::Path<String>) -> JsonResponse<Bucket> {
    let client = cxt.get_ref();
    client.get_bucket(path.as_str()).await
}

#[utoipa::path(
    get,
    path = "/bucket/{bucket_name}/documents",
    tag = "Buckets",
    params(
        (
            "bucket_name" = &str, 
            description = "Passed bucket name to get documents", 
            example = "test_bucket",
        )
    ),
    responses(
        (
            status = 200, 
            description = "Successful", 
            body = [Document], 
            example = json!(vec![Document::test_example()])
        ),
        (
            status = 400, 
            description = "Failed while getting bucket documents", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while getting bucket documents".to_string(),
            })
        ),
    )
)]
#[get("/{bucket_name}/documents")]
async fn get_bucket_documents(cxt: SearcherData, path: web::Path<String>) -> PaginateJsonResponse<Vec<Document>> {
    let client = cxt.get_ref();
    client.get_bucket_documents(path.as_str()).await
}

#[cfg(test)]
mod buckets_endpoints {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::SearcherService;

    use wrappers::bucket::BucketForm;

    use actix_web::test;

    #[test]
    async fn test_create_bucket() {
        let bucket_form = BucketForm::new("test_bucket");
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_bucket(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_delete_bucket() {
        let other_context = OtherContext::new("test".to_string());

        let response = other_context.delete_bucket("test_bucket").await;
        assert_eq!(response.status().as_u16(), 400_u16);

        let bucket_form = BucketForm::new("test_bucket");

        let response = other_context.create_bucket(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let response = other_context.delete_bucket("test_bucket").await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }

    #[test]
    async fn test_get_buckets() {
        let other_context = OtherContext::new("test".to_string());
        let bucket_form = BucketForm::new("test_bucket");
        let response = other_context.create_bucket(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let response = other_context.get_all_buckets().await;
        let buckets_size = response.unwrap().0.len();
        assert_eq!(buckets_size, 1);
    }

    #[test]
    async fn test_get_bucket_by_id() {
        let bucket_form = BucketForm::new("test_bucket");
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_bucket(&bucket_form).await;
        assert_eq!(response.status().as_u16(), 200_u16);

        let get_bucket_result = other_context.get_bucket("test_bucket").await;
        let bucket_uuid = &get_bucket_result.unwrap().uuid;
        assert_eq!(bucket_uuid.as_str(), "test_bucket");
    }
}
