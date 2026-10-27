use anyhow::anyhow;
use openai_dive::v1::error::APIError;
use reqwest::StatusCode;

use crate::application::services::embeddings::TokenizerError;

pub fn map_openai_error(error: APIError) -> TokenizerError {
    match error {
        APIError::ServerError(msg) => TokenizerError::Request(anyhow!(msg)),
        APIError::RateLimitError(msg) => TokenizerError::UnexpectedStatus {
            status: StatusCode::TOO_MANY_REQUESTS.as_u16(),
            body: msg,
        },
        APIError::BadRequestError(msg) => TokenizerError::UnexpectedStatus {
            status: StatusCode::BAD_REQUEST.as_u16(),
            body: msg,
        },
        APIError::AuthenticationError(msg) => TokenizerError::UnexpectedStatus {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            body: msg,
        },
        APIError::PermissionError(msg) => TokenizerError::UnexpectedStatus {
            status: StatusCode::FORBIDDEN.as_u16(),
            body: msg,
        },
        APIError::NotFoundError(msg) => TokenizerError::UnexpectedStatus {
            status: StatusCode::NOT_FOUND.as_u16(),
            body: msg,
        },
        APIError::GoneError(msg) => TokenizerError::UnexpectedStatus {
            status: StatusCode::GONE.as_u16(),
            body: msg,
        },
        APIError::UnknownError(status, msg) => TokenizerError::UnexpectedStatus {
            status,
            body: msg,
        },
        APIError::InvalidRequestError(msg) | APIError::ParseError(msg) => {
            TokenizerError::InvalidResponse(anyhow!(msg))
        },
        APIError::FileError(msg)
        | APIError::StreamError(msg)
        | APIError::WebSocketError(msg) => TokenizerError::Request(anyhow!(msg)),
    }
}
