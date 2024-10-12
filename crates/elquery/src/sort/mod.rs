use derive_builder::Builder;
use serde_derive::Serialize;
use serde_json::{json, Value};

#[derive(Clone, Default, Serialize)]
pub struct SortQuery {
    #[serde(flatten)]
    query: Value,
}

impl SortQuery {
    pub fn with_must_field(mut self, key: &str, value: SortItem) -> Self {
        let sort_item_value = json!({ key: value });
        self.query = sort_item_value;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortItemFormat {
    #[default]
    #[serde(rename = "strict_date_optional_time_nanos")]
    StrictDateOptionalTimeNanos,
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortItemOrder {
    #[serde(rename = "acs")]
    Asc,
    #[default]
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Builder, Clone, Default, Serialize)]
pub struct SortItem {
    order: SortItemOrder,
    format: SortItemFormat,
}

impl SortItem {
    pub fn with_order(mut self, order: SortItemOrder) -> Self {
        self.order = order;
        self
    }

    pub fn with_format(mut self, format: SortItemFormat) -> Self {
        self.format = format;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
