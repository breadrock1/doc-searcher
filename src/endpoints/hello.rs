use crate::errors::{ErrorResponse, JsonResponse, Successful};

use actix_web::get;
use actix_web::web::Json;

#[utoipa::path(
    get,
    path = "/hello/",
    tag = "Hello",
    responses(
        (
            status = 200,
            description = "Successful",
            body = Successful,
            example = json!(Successful {
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
async fn hello() -> JsonResponse<Successful> {
    Ok(Json(Successful::success("Ok")))
}
