use crate::errors::{ErrorResponse, Successful};

pub trait TestExample<T> {
    fn test_example(value: Option<&str>) -> T;
}

impl TestExample<Successful> for Successful {
    fn test_example(value: Option<&str>) -> Successful {
        let msg = value.unwrap_or("Done");
        Successful::new(200, msg)
    }
}

impl TestExample<ErrorResponse> for ErrorResponse {
    fn test_example(value: Option<&str>) -> ErrorResponse {
        let msg = value.unwrap_or("bad client request");
        ErrorResponse::new(400, "Bad request", msg)
    }
}
