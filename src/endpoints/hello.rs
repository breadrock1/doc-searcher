use crate::endpoints::ContextData;
use crate::errors::SuccessfulResponse;

use actix_web::{get, HttpResponse};

#[get("/hello")]
async fn hello(cxt: ContextData) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}
