use serde_derive::Serialize;
use serde_json::{json, Value};

#[derive(Clone, Serialize)]
pub struct BoolMatchAllQuery {
    match_all: Value,
}

impl Default for BoolMatchAllQuery {
    fn default() -> Self {
        BoolMatchAllQuery {
            match_all: json!({})
        }
    }
}
