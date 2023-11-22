use crate::errors::SuccessfulResponse;
use crate::searcher::service_client::ServiceClient;

use actix_web::{get, web, HttpResponse};

#[get("/hello")]
async fn hello(cxt: web::Data<&dyn ServiceClient>) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}
