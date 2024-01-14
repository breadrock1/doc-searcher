use crate::endpoints::ContextData;
use crate::errors::SuccessfulResponse;
use crate::wrappers::file_form::{LoadFileForm, UploadFileForm};

use actix_multipart::form::MultipartForm;
use actix_web::{post, web, HttpResponse};

#[post("/loader/load")]
async fn load_file(cxt: ContextData, form: web::Json<LoadFileForm>) -> HttpResponse {
    let client = cxt.get_ref();
    let file_path = form.get_path();
    let bucket_name = form.get_bucket();
    client.load_file_to_bucket(bucket_name, file_path).await
}
