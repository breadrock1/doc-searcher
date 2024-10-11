use derive_builder::Builder;
use serde_derive::Serialize;
use serde_json::{json, Value};

#[derive(Clone, Default, Serialize)]
pub struct SortQuery {
    must: Vec<Value>,
}

impl SortQuery {
    pub fn with_field(mut self, key: &str, value: SortItem) -> Self {
        let sort_item_value = json!({ key: value });
        self.must.push(sort_item_value);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Builder, Clone, Default, Serialize)]
pub struct SortItem {
    order: SortItemOrder,
    format: SortItemFormat,
}

impl SortItem {
    pub fn builder() -> SortItemBuilder {
        SortItemBuilder::default()
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
