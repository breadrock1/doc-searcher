use crate::endpoints::SearcherData;
use crate::errors::{ErrorResponse, SuccessfulResponse};

use actix_web::{post, web};
use actix_web::{HttpRequest, HttpResponse};

use wrappers::file_form::LoadFileForm;

#[utoipa::path(
    post,
    path = "/file/load",
    tag = "Files",
    request_body(
        content = LoadFileForm,
        example = json!({
            "file_path": "/tmp/test.txt",
            "bucket_name": "test_bucket",
        }),
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
            description = "Failed while loading files", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while loading files".to_string(),
            })
        ),
    )
)]
#[post("/load")]
async fn load_file(cxt: SearcherData, form: web::Json<LoadFileForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    let bucket_name = form.get_bucket();
    client.load_file_to_bucket(bucket_name, file_path).await
}

#[utoipa::path(
    post,
    path = "/file/download",
    tag = "Files",
    request_body(
        content = LoadFileForm,
        example = json!({
            "file_path": "/tmp/test.txt",
            "bucket_name": "test_bucket",
        }),
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
            description = "Failed while downloading files", 
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 400,
                error: "Bad Request".to_string(),
                message: "Failed while downloading files".to_string(),
            })
        ),
    )
)]
#[post("/download")]
async fn download_file(
    cxt: SearcherData,
    req: HttpRequest,
    form: web::Json<LoadFileForm>,
) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    let bucket_name = form.get_bucket();
    client
        .download_file(bucket_name, file_path)
        .await
        .unwrap()
        .into_response(&req)
}

#[cfg(test)]
mod loader_endpoints {
    use crate::services::own_engine::context::OtherContext;
    use crate::services::SearcherService;

    use actix_web::test;

    #[test]
    async fn test_load_file() {
        let file_path = "src/crates/loader/resources/test.txt";
        let other_context = OtherContext::new("test".to_string());
        let response = other_context
            .load_file_to_bucket("test_bucket", file_path)
            .await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }
}
