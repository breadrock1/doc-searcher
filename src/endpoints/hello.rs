use crate::endpoints::SearcherData;
use crate::errors::{ErrorResponse, SuccessfulResponse};

use actix_web::{get, HttpResponse};

#[utoipa::path(
    get,
    path = "/hello",
    tag = "Hello",
    responses(
        (
            status = 200,
            description = "Successful",
            body = SuccessfulResponse,
            example = json!(SuccessfulResponse {
                code: 200,
                message: "Hello".to_string(),
            })
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse {
                code: 503,
                error: "Server error".to_string(),
                message: "Server does not available".to_string(),
            })
        )
    ),
)]
#[get("/")]
async fn hello(cxt: SearcherData) -> HttpResponse {
    let _client = cxt.get_ref();
    SuccessfulResponse::ok_response("Ok")
}
