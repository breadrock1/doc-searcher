use crate::filter::FilterQueryTrait;
use crate::search::match_all_query::BoolMatchAllQuery;
use crate::search::SearchQueryTrait;

use serde_derive::Serialize;
use serde_json::Value;

#[derive(Clone, Default, Serialize)]
pub struct BoolQuery {
    bool: BoolQueryItems,
}

impl BoolQuery {
    pub fn with_match_all(mut self, query_type: BoolQueryType) -> Self {
        let must_all_query = BoolMatchAllQuery::default();
        let must_all_value = serde_json::to_value(must_all_query).ok();

        match query_type {
            BoolQueryType::Match => {
                self.bool.match_ = must_all_value;
                self.bool.must = None;
                self.bool.should = None;
            }
            BoolQueryType::Must => {
                self.bool.must = must_all_value;
                self.bool.match_ = None;
                self.bool.should = None;
            },
            BoolQueryType::Should => {
                self.bool.should = must_all_value;
                self.bool.must = None;
                self.bool.match_ = None;
            }
        }

        self
    }

    pub fn with_query<T>(mut self, query: T, query_type: BoolQueryType) -> Self
    where
        T: SearchQueryTrait + serde::Serialize
    {
        let query_value = serde_json::to_value(query).unwrap();
        match query_type {
            BoolQueryType::Match => {
                self.bool.match_ = Some(query_value);
                self.bool.must = None;
                self.bool.should = None;
            }
            BoolQueryType::Must => {
                self.bool.must = Some(query_value);
                self.bool.match_ = None;
                self.bool.should = None;
            },
            BoolQueryType::Should => {
                self.bool.should = Some(query_value);
                self.bool.must = None;
                self.bool.match_ = None;
            }
        }

        self
    }

    pub fn with_filter<T>(mut self, filter: T) -> Self
    where
        T: FilterQueryTrait + serde::Serialize,
    {
        let filter_value = serde_json::to_value(filter).unwrap();
        let common_filter = CommonFilter { bool: filter_value };
        self.bool.filter = Some(common_filter);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

pub enum BoolQueryType {
    Match,
    Must,
    Should,
}

#[derive(Clone, Default, Serialize)]
struct BoolQueryItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    must: Option<Value>,
    #[serde(rename = "match")]
    #[serde(skip_serializing_if = "Option::is_none")]
    match_: Option<Value>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    should: Option<Value>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    more_like_this: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<CommonFilter>,
}

#[derive(Clone, Default, Serialize)]
struct CommonFilter {
    bool: Value,
}
