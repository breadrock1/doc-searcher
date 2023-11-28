use crate::endpoints::ContextData;
use crate::errors::SuccessfulResponse;
use crate::wrappers::file_form::{LoadFileForm, UploadFileForm};

use actix_multipart::form::MultipartForm;
use actix_web::{post, web, HttpResponse};

#[post("/loader/upload")]
async fn upload_file(cxt: ContextData, _form: MultipartForm<UploadFileForm>) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}

#[post("/loader/{bucket_name}/upload")]
async fn upload_file_to_bucket(
    cxt: ContextData,
    _form: MultipartForm<UploadFileForm>,
) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}

#[post("/loader/load")]
async fn load_file(cxt: ContextData, form: web::Json<LoadFileForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    client.load_file_to_all(file_path).await
}

#[post("/loader/{bucket_name}/load")]
async fn load_file_to_bucket(cxt: ContextData, form: web::Json<LoadFileForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    match form.get_bucket() {
        None => client.load_file_to_all(file_path),
        Some(bucket) => client.load_file_to_bucket(bucket, file_path),
    }
    .await
}
