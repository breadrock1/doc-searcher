use crate::search::should_query::{MatchItemQuery, ShouldMatchItemTrait};

use serde_derive::Serialize;
use serde_json::Value;

pub trait MultiMatchItemTrait {}
impl MultiMatchItemTrait for MatchItemQuery {}

#[derive(Clone, Default, Serialize)]
pub struct BoolMultiMatchQuery {
    multi_match: Value,
}

impl BoolMultiMatchQuery {
    pub fn set_item<T>(mut self, item: T) -> Self
    where
        T: ShouldMatchItemTrait + serde::Serialize,
    {
        let value = serde_json::to_value(item).unwrap();
        self.multi_match = value;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
