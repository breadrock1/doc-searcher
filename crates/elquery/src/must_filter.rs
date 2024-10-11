use crate::filter_query::{DocumentSizeQuery, FilterDateQuery, FilterItem, FilterMust};

use serde_derive::Serialize;
use serde_json::json;

#[derive(Clone, Default, Serialize)]
pub struct CommonMustFilter {
    bool: FilterMust,
}

impl CommonMustFilter {
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

    pub fn with_exists<T>(mut self, value: &str) -> Self
    where
        T: FilterItem + serde::Serialize,
    {
        let exists_field = json!({"field": value});
        let filter_exists = T::create(exists_field);
        let serde_result = serde_json::to_value(filter_exists);
        if serde_result.is_ok() {
            let filter_value = serde_result.unwrap();
            self.bool.must.push(filter_value);
        }

        self
    }

    pub fn build(self) -> Self {
        self
    }
}
