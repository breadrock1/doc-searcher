use crate::errors::{ErrorResponse, JsonResponse, Successful};

use actix_web::{get, Scope, web};
use actix_web::web::Json;

pub fn build_scope() -> Scope {
    web::scope("/metrics").service(metrics)
}

#[utoipa::path(
    get,
    path = "/metrics/",
    tag = "Metrics",
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
                attachments: None,
            })
        )
    ),
)]
#[get("/")]
async fn metrics() -> JsonResponse<Successful> {
    Ok(Json(Successful::success("Ok")))
}
