use crate::context::SearchContext;
use crate::errors::SuccessfulResponse;

use actix_web::{get, web, HttpResponse};

#[get("/hello")]
async fn hello(cxt: web::Data<SearchContext>) -> HttpResponse {
    let _elastic = cxt.get_cxt().read().await;
    SuccessfulResponse::ok_response("Ok")
}
