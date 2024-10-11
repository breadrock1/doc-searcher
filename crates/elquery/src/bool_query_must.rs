use crate::filter_query::FilterDateQuery;
use serde_derive::Serialize;
use serde_json::{json, Value};

pub struct BoolQueryMust {
    must: Vec<Value>,
}

pub trait MustItemFilter {
    fn create(value: Value) -> Value;
}

impl BoolQueryMust {
    pub fn with_term<T>(mut self, key: &str, value: Option<T>) -> Self
    where
        T: MustItemFilter + serde::Serialize
    {
        if let Some(val) = value {
            let pair_value = json!({key: val});
            let term_value = MustFilterTerm::create(pair_value);
            self.must.push(term_value);
        }

        self
    }

    pub fn with_match<T>(mut self, key: &str, value: Option<T>) -> Self
    where
        T: MustItemFilter + serde::Serialize,
    {
        if let Some(val) = value {
            let pair_value = json!({key: val});
            let term_value = MustFilterMatch::create(pair_value);
            self.must.push(term_value);
        }

        self
    }

    pub fn with_range<T>(mut self, key: &str, gte: i64, lte: i64) -> Self
    where
        T: MustItemFilter + serde::Serialize,
    {
        // if let Some(val) = value {
        //     let pair_value = json!({key: val});
        //     let term_value = MustFilterRange::create(pair_value);
        //     self.must.push(term_value);
        // }

        self
    }

    pub fn with_exists(mut self, value: Option<&str>) -> Self {
        if let Some(val) = value {
            let pair_value = json!({"field": val});
            let exists_value = MustFilterExists::create(pair_value);
            self.must.push(exists_value);
        }

        self
    }

    pub fn with_date<T, U>(mut self, key: &str, gte: &str, lte: &str) -> Self
    where
        T: MustItemFilter + serde::Serialize,
        U: FilterDateQuery + serde::Serialize,
    {
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Default, Serialize)]
pub struct MustFilterTerm {
    term: Option<Value>,
}

impl MustItemFilter for MustFilterTerm {
    fn create(value: Value) -> Value {
        let must_term_filter = MustFilterTerm { term: Some(value) };
        serde_json::to_value(must_term_filter).unwrap()
    }
}

#[derive(Default, Serialize)]
pub struct MustFilterMatch {
    #[serde(rename = "match")]
    match_value: Option<Value>,
}

impl MustItemFilter for MustFilterMatch {
    fn create(value: Value) -> Value {
        let must_filter_match = MustFilterMatch { match_value: Some(value) };
        serde_json::to_value(must_filter_match).unwrap()
    }
}

#[derive(Default, Serialize)]
pub struct MustFilterRange {
    range: Option<Value>,
}

impl MustItemFilter for MustFilterRange {
    fn create(value: Value) -> Value {
        let must_filter_range = MustFilterRange { range: Some(value) };
        serde_json::to_value(must_filter_range).unwrap()
    }
}

#[derive(Default, Serialize)]
pub struct MustFilterExists {
    exists: Option<Value>,
}

impl MustItemFilter for MustFilterExists {
    fn create(value: Value) -> Value {
        let must_filter_exists = MustFilterExists { exists: Some(value) };
        serde_json::to_value(must_filter_exists).unwrap()
    }
}

#[derive(Default, Serialize)]
pub struct MustFilterDate {
    gte: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<String>,
}
