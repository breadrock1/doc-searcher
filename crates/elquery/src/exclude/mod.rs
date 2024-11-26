use serde_derive::Serialize;

#[derive(Clone, Default, Serialize)]
pub struct ExcludeFields {
    exclude: Vec<String>,
}

impl ExcludeFields {
    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.exclude.extend_from_slice(fields.as_slice());
        self
    }

    pub fn append_field(mut self, field: String) -> Self {
        self.exclude.push(field);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
