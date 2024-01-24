use crate::endpoints::ContextData;
use crate::wrappers::file_form::LoadFileForm;

use actix_web::{post, web};
use actix_web::{HttpRequest, HttpResponse};

#[utoipa::path(
    post,
    path = "/searcher/loader/load",
    tag = "Load file from local file system of service by path",
    request_body = LoadFileForm,
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 401, description = "Failed while loading files", body = ErrorResponse),
    )
)]
#[post("/loader/load")]
async fn load_file(cxt: ContextData, form: web::Json<LoadFileForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    let bucket_name = form.get_bucket();
    client.load_file_to_bucket(bucket_name, file_path).await
}

#[utoipa::path(
    post,
    path = "/searcher/loader/load-file",
    tag = "Download file from local system by path",
    request_body = LoadFileForm,
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 401, description = "Failed while downloading files", body = ErrorResponse),
    )
)]
#[post("/loader/load-file")]
async fn download_file(
    cxt: ContextData,
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
    use crate::service::own_engine::context::OtherContext;
    use crate::service::ServiceClient;

    use actix_web::test;

    #[test]
    async fn test_load_file() {
        let file_path = "src/crates/loader/resources/test.txt";
        let other_context = OtherContext::_new("test".to_string());
        let response = other_context
            .load_file_to_bucket("test_bucket", file_path)
            .await;
        assert_eq!(response.status().as_u16(), 200_u16);
    }
}
