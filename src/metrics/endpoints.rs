use crate::errors::{ErrorResponse, JsonResponse, Successful};

use actix_web::web::Json;
use actix_web::{get, web, HttpResponse, Scope};

pub fn build_scope() -> Scope {
    let scope = web::scope("/metrics").service(hello);

    #[cfg(feature = "enable-prometheus")]
    let scope = scope.service(metrics);

    scope
}

#[utoipa::path(
    get,
    path = "/metrics/hello",
    tag = "metrics",
    description = "Get current service metrics",
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
#[get("/hello")]
async fn hello() -> JsonResponse<Successful> {
    Ok(Json(Successful::new(200, "Ok")))
}

#[get("/metrics")]
async fn metrics() -> HttpResponse {
    HttpResponse::Ok().finish()
}
