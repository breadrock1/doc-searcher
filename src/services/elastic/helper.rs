use crate::errors::{SuccessfulResponse, WebError, WebResult};

use elasticsearch::http::response::Response;
use std::string::ToString;

pub(crate) async fn parse_elastic_response(response: Response) -> WebResult {
    if !response.status_code().is_success() {
        return Err(extract_exception(response).await);
    }

    Ok(SuccessfulResponse::success("Ok"))
}

pub(crate) async fn extract_exception(response: Response) -> WebError {
    let exception_opt = response.exception().await.map_err(WebError::from).unwrap();
    match exception_opt {
        None => WebError::UnknownError("Unknown error".to_string()),
        Some(exception) => WebError::from(exception),
    }
}
