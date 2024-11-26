use serde_derive::Serialize;
use serde_json::Value;

#[derive(Clone, Default, Serialize)]
pub struct BoolShouldFilter {
    should: Vec<Value>,
}
