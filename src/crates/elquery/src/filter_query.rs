use serde_derive::Serialize;
use serde_json::{json, Value};

#[derive(Clone, Default, Serialize)]
pub struct CommonFilter {
    bool: FilterMust,
}

impl CommonFilter {
    pub fn new() -> Self {
        CommonFilter::default()
    }

    pub fn with_term<T>(mut self, key: &str, param: &str) -> Self
    where
        T: FilterItem + serde::Serialize,
    {
        if !param.is_empty() {
            let value = json!({ key: param });
            let filter_term = T::create(value);
            let serde_result = serde_json::to_value(filter_term);
            if serde_result.is_ok() {
                let filter_value = serde_result.unwrap();
                self.bool.must.push(filter_value);
            }
        }

        self
    }

    pub fn with_match<T>(mut self, key: &str, param: &str) -> Self
    where
        T: FilterItem + serde::Serialize,
    {
        if !param.is_empty() {
            let value = json!({ key: param });
            let filter_match = T::create(value);
            let serde_result = serde_json::to_value(filter_match);
            if serde_result.is_ok() {
                let filter_value = serde_result.unwrap();
                self.bool.must.push(filter_value);
            }
        }

        self
    }

    pub fn with_range<T>(mut self, key: &str, gte: i64, lte: i64) -> Self
    where
        T: FilterItem + serde::Serialize,
    {
        let doc_size_query = DocumentSizeQuery::new(gte, lte);
        let value = json!({ key: doc_size_query });
        let filter_range = T::create(value);
        let serde_result = serde_json::to_value(filter_range);
        if serde_result.is_ok() {
            let filter_value = serde_result.unwrap();
            self.bool.must.push(filter_value);
        }

        self
    }

    pub fn with_date<T, U>(mut self, key: &str, gte: &str, lte: &str) -> Self
    where
        T: FilterItem + serde::Serialize,
        U: FilterDateQuery + serde::Serialize,
    {
        if !gte.is_empty() {
            let doc_date_query = U::new(gte, lte);
            let value = json!({ key: doc_date_query });
            let filter_range = T::create(value);
            let serde_result = serde_json::to_value(filter_range);
            if serde_result.is_ok() {
                let filter_value = serde_result.unwrap();
                self.bool.must.push(filter_value);
            }
        }

        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Clone, Default, Serialize)]
struct FilterMust {
    must: Vec<Value>,
}

pub trait FilterItem {
    fn create(value: Value) -> Self;
}

#[derive(Clone, Default, Serialize)]
pub struct FilterTerm {
    term: Option<Value>,
}

impl FilterItem for FilterTerm {
    fn create(value: Value) -> Self {
        FilterTerm { term: Some(value) }
    }
}

#[derive(Clone, Default, Serialize)]
pub struct FilterRange {
    range: Option<Value>,
}

impl FilterItem for FilterRange {
    fn create(value: Value) -> Self {
        FilterRange { range: Some(value) }
    }
}

#[derive(Clone, Default, Serialize)]
pub struct FilterPrefix {
    prefix: Option<Value>,
}

impl FilterItem for FilterPrefix {
    fn create(value: Value) -> Self {
        FilterPrefix {
            prefix: Some(value),
        }
    }
}

#[derive(Clone, Default, Serialize)]
pub struct FilterMatch {
    #[serde(rename = "match")]
    match_value: Option<Value>,
}

impl FilterItem for FilterMatch {
    fn create(value: Value) -> Self {
        FilterMatch {
            match_value: Some(value),
        }
    }
}

pub trait FilterDateQuery {
    fn new(gte: &str, lte: &str) -> Self;
}

#[derive(Serialize)]
#[serde(rename = "created_at")]
pub struct CreatedAtDateQuery {
    gte: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<String>,
}

impl FilterDateQuery for CreatedAtDateQuery {
    fn new(gte: &str, _lte: &str) -> Self {
        CreatedAtDateQuery {
            gte: gte.to_string(),
            lte: None,
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "document_created")]
pub struct CreateDateQuery {
    gte: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<String>,
}

impl FilterDateQuery for CreateDateQuery {
    fn new(gte: &str, lte: &str) -> Self {
        let lte_value = match lte.is_empty() {
            false => Some(lte.to_string()),
            true => Some("now/d".to_string()),
        };

        CreateDateQuery {
            gte: gte.to_string(),
            lte: lte_value,
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "document_modified")]
pub struct ModifyDateQuery {
    gte: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<String>,
}

impl FilterDateQuery for ModifyDateQuery {
    fn new(gte: &str, lte: &str) -> Self {
        let lte_value = match lte.is_empty() {
            true => Some(lte.to_string()),
            false => None,
        };

        ModifyDateQuery {
            gte: gte.to_string(),
            lte: lte_value,
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "document_size")]
pub struct DocumentSizeQuery {
    gte: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<i64>,
}

impl DocumentSizeQuery {
    pub fn new(gte: i64, lte: i64) -> Self {
        let lte_value = match lte > 0 {
            true => Some(lte),
            false => None,
        };

        DocumentSizeQuery {
            gte,
            lte: lte_value,
        }
    }
}
