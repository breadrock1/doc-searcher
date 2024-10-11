use crate::filter_query::FilterItem;

use serde_derive::Serialize;
use serde_json::Value;

#[derive(Clone, Default, Serialize)]
pub struct CommonShouldFilter {
    should: Vec<Value>,
}

impl CommonShouldFilter {
    pub fn with_multi_match<T>(mut self, filter: T) -> Self
    where
        T: FilterItem + serde::Serialize,
    {
        let value = serde_json::to_value(filter).unwrap();
        self.should.push(value);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
