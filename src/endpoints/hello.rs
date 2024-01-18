use crate::endpoints::ContextData;
use crate::errors::*;

use actix_web::{get, HttpResponse};

#[utoipa::path(
    get,
    path = "/searcher/hello",
    tag = "Test server connection endpoint",
    responses(
        (status = 200, description = "Successful", body = SuccessfulResponse),
        (status = 501, description = "Server does not available", body = ErrorResponse),
    ),
)]
#[get("/hello")]
async fn hello(cxt: ContextData) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}
