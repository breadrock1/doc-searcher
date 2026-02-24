use serde_json::json;
use serde_json::Value;

use super::constants::TEST_INDEX_ID;

pub fn created_index_json_object() -> Value {
    json!({
        "id": TEST_INDEX_ID,
    })
}

pub fn get_all_indexes_json_object() -> Value {
    json!(vec![created_index_json_object()])
}
