use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::application::services::server::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            status: u16,
            message: String,
        }

        let (status, msg) = self.status_code();
        tracing::error!(status=%status, msg=%msg, "error response");
        let response = ErrorResponse {
            status: status.as_u16(),
            message: msg.to_string(),
        };
        let mut resp = Json(response).into_response();
        *resp.status_mut() = status;
        resp
    }
}
