use serde_derive::Serialize;
use serde_json::{json, Value};

trait MustFilterItemTrait {}
impl MustFilterItemTrait for TermFilterItem {}
impl MustFilterItemTrait for RangeFilterItem {}

#[derive(Default, Serialize)]
pub struct BoolMustFilter {
    must: Vec<Value>,
}

impl BoolMustFilter {
    pub fn with_term<T>(mut self, key: &str, value: T) -> Self
    where
        T: serde::Serialize,
    {
        let term_item = TermFilterItem::term_value(key, value);
        let term_item_val = serde_json::to_value(term_item).unwrap();
        self.must.push(term_item_val);
        self
    }

    pub fn with_range<T, U>(mut self, key: &str, gte: T, lte: Option<U>) -> Self
    where
        T: serde::Serialize,
        U: serde::Serialize,
    {
        let range_item = RangeFilterItem::range_value(gte, lte);
        let range_value = json!({"range": { key: range_item }});
        self.must.push(range_value);
        self
    }

    pub fn with_exists(mut self, field: &str) -> Self {
        let exists_query = ExistsFilterItem::exists_value(field);
        let exists_query_val = serde_json::to_value(exists_query).unwrap();
        self.must.push(exists_query_val);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Default, Serialize)]
struct TermFilterItem {
    term: Value,
}

impl TermFilterItem {
    pub fn term_value<T>(key: &str, value: T) -> Self
    where
        T: serde::Serialize,
    {
        let term_item = json!({key: value});
        TermFilterItem {
            term: term_item,
        }
    }
}

#[derive(Default, Serialize)]
struct RangeFilterItem {
    gte: Value,
    lte: Option<Value>,
}

impl RangeFilterItem {
    pub fn range_value<T, U>(gte: T, lte: Option<U>) -> Self
    where
        T: serde::Serialize,
        U: serde::Serialize,
    {
        let gte_value = serde_json::to_value(gte).unwrap();
        let lte_value = lte.map_or_else(
            || None,
            |val| serde_json::to_value(val).ok(),
        );

        RangeFilterItem {
            gte: gte_value,
            lte: lte_value,
        }
    }
}

#[derive(Default, Serialize)]
struct ExistsFilterItem {
    exists: Value,
}

impl ExistsFilterItem {
    pub fn exists_value(field: &str) -> Self {
        let exists_value = json!({"field": field});
        ExistsFilterItem {
            exists: exists_value,
        }
    }
}
