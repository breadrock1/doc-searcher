use serde_derive::Serialize;

#[derive(Clone, Default, Serialize)]
pub struct BoolMustQuery {
    multi_match: MultiMatchQuery,
}

impl BoolMustQuery {
    pub fn with_query(mut self, query: &str) -> Self {
        self.multi_match.query = query.to_string();
        self
    }

    pub fn with_operator(mut self, operator: Option<MultiMatchOperator>) -> Self {
        self.multi_match.operator = operator;
        self
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.multi_match.fields.extend_from_slice(fields.as_slice());
        self
    }

    pub fn append_field(mut self, field: &str) -> Self {
        self.multi_match.fields.push(field.to_string());
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MultiMatchOperator {
    #[default]
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "and")]
    And,
}

#[derive(Clone, Default, Serialize)]
struct MultiMatchQuery {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<MultiMatchOperator>,
    fields: Vec<String>,
}
