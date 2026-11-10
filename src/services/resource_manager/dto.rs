use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResourceSchema {
    pub(crate) external_id: Option<String>,
    #[serde(default)]
    pub(crate) prefix: String,
}
