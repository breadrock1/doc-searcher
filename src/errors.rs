use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use elasticsearch::Error;
use serde::Serialize;
use thiserror::Error;

pub type WebResponse<T> = Result<T, WebError>;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Some error")]
    SomeError(String),
}

impl WebError {
    pub fn new(msg: String) -> Self {
        WebError::SomeError(msg)
    }

    pub fn name(&self) -> String {
        match self {
            WebError::SomeError(_) => "",
        }
        .to_string()
    }
}

impl From<elasticsearch::Error> for WebError {
    fn from(value: Error) -> Self {
        WebError::new(value.to_string())
    }
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::SomeError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };

        HttpResponse::build(status_code).json(response)
    }
}


#[derive(Serialize)]
pub struct Successful {
    code: u16,
    message: String,
}

impl Successful {
    pub fn ok_response(msg: &str) -> HttpResponse {
        let status_code = StatusCode::OK;
        let response = Successful {
            code: status_code.as_u16(),
            message: msg.to_string(),
        };
        HttpResponse::build(status_code).json(response)
    }
}
