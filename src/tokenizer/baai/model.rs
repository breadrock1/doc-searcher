use derive_builder::Builder;
use serde_derive::Serialize;

#[derive(Builder, Serialize)]
pub struct BAAIForm {
    inputs: String,
    truncate: bool,
    normalize: bool,
}

impl BAAIForm {
    pub fn builder() -> BAAIFormBuilder {
        BAAIFormBuilder::default()
    }
}
