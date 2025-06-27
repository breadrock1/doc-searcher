use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::application::services::server::error::ServerError;
use crate::infrastructure::httpserver::swagger::SwaggerExample;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (msg, status) = self.status_code();
        let mut resp = Json(ErrorResponse {
            message: msg.to_string(),
        })
        .into_response();

        *resp.status_mut() = status;
        resp
    }
}

impl SwaggerExample for ServerError {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        match value {
            None => ServerError::ServiceUnavailable("service unavailable".to_owned()),
            Some(msg) => ServerError::InternalError(msg.to_owned()),
        }
    }
}
