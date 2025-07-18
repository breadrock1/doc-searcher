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
        let response = ErrorResponse {
            status: status.as_u16(),
            message: msg,
        };
        let mut resp = Json(response).into_response();
        *resp.status_mut() = status;
        resp
    }
}
