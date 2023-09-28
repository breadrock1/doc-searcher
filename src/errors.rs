use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

pub type WebResponse<T> = Result<T, Error>;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Some error")]
    SomeError(String),
}

impl Error {
    pub fn name(&self) -> String {
        match self {
            Error::SomeError(_) => "",
        }
        .to_string()
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::SomeError(_) => StatusCode::BAD_REQUEST,
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
