use crate::exclude::ExcludeFields;
use crate::highlight::HighlightQuery;
use crate::r#match::BoolQuery;

use derive_builder::Builder;
use serde_derive::Serialize;
use serde_json::Value;

pub mod exclude;
pub mod filter;
pub mod highlight;
pub mod r#match;
pub mod search;
mod similar;
pub mod sort;

#[derive(Builder, Clone, Default, Serialize)]
pub struct CommonQuery {
    query: BoolQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlight: Option<HighlightQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    _source: Option<ExcludeFields>,
}

impl CommonQuery {
    pub fn builder() -> CommonQueryBuilder {
        CommonQueryBuilder::default()
    }
}
