use crate::errors::{ErrorResponse, JsonResponse, Successful};

use actix_web::web::Json;
use actix_web::{get, web, Scope};

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
            example = json!(Successful::new(200, "Hello")),
        ),
        (
            status = 503,
            description = "Server does not available",
            body = ErrorResponse,
            example = json!(ErrorResponse::new(503, "Server error", "Server does not available")),
        ),
    ),
)]
#[get("/")]
async fn metrics() -> JsonResponse<Successful> {
    Ok(Json(Successful::new(200, "Ok")))
}
