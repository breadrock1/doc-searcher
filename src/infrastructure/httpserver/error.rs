use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::application::services::server::ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (msg, status) = self.status_code();
        let mut resp = Json(ErrorResponse { message: msg }).into_response();
        *resp.status_mut() = status;
        resp
    }
}
