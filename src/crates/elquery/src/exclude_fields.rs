use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ExcludeFields {
    exclude: Option<Vec<String>>,
}

impl ExcludeFields {
    pub fn new(values: Option<Vec<String>>) -> Self {
        ExcludeFields { exclude: values }
    }
}
