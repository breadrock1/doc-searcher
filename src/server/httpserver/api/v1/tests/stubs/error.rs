use serde_json::json;
use serde_json::Value;

pub fn success_json_response() -> Value {
    json!({
        "status": 200,
        "message": "ok",
    })
}

pub fn bad_request_error_json_response() -> Value {
    json!({
        "status": 400,
        "message": "bad request",
    })
}

pub fn not_found_error_json_response() -> Value {
    json!({
        "status": 404,
        "message": "not found",
    })
}

pub fn conflict_error_json_response() -> Value {
    json!({
        "status": 409,
        "message": "conflict error",
    })
}

pub fn internal_server_error_json_response() -> Value {
    json!({
        "status": 500,
        "message": "internal server error",
    })
}
