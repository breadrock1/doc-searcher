use derive_builder::Builder;
use serde_derive::Serialize;
use serde_json::Value;

pub trait FilterTrait {

}

#[derive(Clone, Default, Serialize)]
pub struct FilterMust {
    pub must: Vec<Value>,
}

impl FilterTrait for FilterMust {}

#[derive(Clone, Default, Serialize)]
pub struct FilterShould {
    pub should: Vec<Value>,
}

impl FilterTrait for FilterShould {}


pub trait FilterItem {
    fn create(value: Value) -> Self;
}

#[derive(Clone, Default, Serialize)]
pub struct FilterExists {
    exists: Value,
}

impl FilterItem for FilterExists {
    fn create(value: Value) -> Self {

        FilterExists {
            exists: value,
        }
    }
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
pub struct FilterMultiMatch {
    multi_match: Option<Value>,
}

impl FilterItem for FilterMultiMatch {
    fn create(value: Value) -> Self {
        FilterMultiMatch {
            multi_match: Some(value),
        }
    }
}

#[derive(Builder, Clone, Default, Serialize)]
pub struct FilterMultiMatchItem {
    query: String,
    item_type: FilterMultiMatchType,
    fields: Vec<String>,
    minimum_should_match: String,
}

impl FilterMultiMatchItem {
    pub fn builder() -> FilterMultiMatchItemBuilder {
        FilterMultiMatchItemBuilder::default()
    }
}

impl FilterItem for FilterMultiMatchItem {
    fn create(value: Value) -> Self {
        unimplemented!()
    }
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterMultiMatchType {
    #[default]
    #[serde(rename = "phrase_prefix")]
    PhrasePrefix,
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
