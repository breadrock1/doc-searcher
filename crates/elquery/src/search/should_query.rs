use crate::search::multi_match_query::BoolMultiMatchQuery;

use derive_builder::Builder;
use serde_derive::Serialize;
use serde_json::Value;

pub trait ShouldMatchItemTrait {}
impl ShouldMatchItemTrait for MatchItemQuery {}
impl ShouldMatchItemTrait for BoolMultiMatchQuery {}

#[derive(Clone, Default, Serialize)]
pub struct BoolShouldQuery {
    should: Vec<Value>,
}

impl BoolShouldQuery {
    pub fn with_items<T>(mut self, items: Vec<T>) -> Self
    where
        T: ShouldMatchItemTrait + serde::Serialize,
    {
        let values = items
            .into_iter()
            .map(serde_json::to_value)
            .map(Result::unwrap)
            .collect::<Vec<Value>>();

        self.should.extend_from_slice(values.as_slice());
        self
    }

    pub fn append_item<T>(mut self, item: T) -> Self
    where
        T: ShouldMatchItemTrait + serde::Serialize,
    {
        let value = serde_json::to_value(item).unwrap();
        self.should.push(value);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchItemType {
    #[default]
    #[serde(rename = "phrase_prefix")]
    PhrasePrefix,
}

#[derive(Builder, Default, Serialize)]
pub struct MatchItemQuery {
    query: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    item_type: Option<MatchItemType>,
    /// Setup [1, 100] integer value like string "50%"
    minimum_should_match: String,
    fields: Vec<String>,
}

impl MatchItemQuery {
    pub fn builder() -> MatchItemQueryBuilder {
        MatchItemQueryBuilder::default()
    }
}
