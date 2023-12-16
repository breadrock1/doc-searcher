use serde_derive::Serialize;

#[derive(Serialize)]
struct QueryString {
    query: String,
}

impl QueryString {
    pub fn new(value: &str) -> Self {
        QueryString {
            query: value.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct MultiMatchQuery {
    multi_match: QueryString,
}

impl MultiMatchQuery {
    pub fn new(value: &str) -> Self {
        MultiMatchQuery {
            multi_match: QueryString::new(value),
        }
    }
}
