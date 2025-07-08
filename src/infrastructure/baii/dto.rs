use derive_builder::Builder;
use serde_derive::Serialize;

#[derive(Builder, Serialize)]
pub struct VectorizerForm {
    inputs: String,
    truncate: bool,
    normalize: bool,
}

impl VectorizerForm {
    pub fn builder() -> VectorizerFormBuilder {
        VectorizerFormBuilder::default()
    }
}
