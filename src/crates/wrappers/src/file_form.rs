use crate::bucket::DEFAULT_BUCKET_NAME;

use serde_derive::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct LoadFileForm {
    file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<String>,
}

impl LoadFileForm {
    pub fn get_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn get_bucket(&self) -> &str {
        match self.bucket_name.as_ref() {
            None => DEFAULT_BUCKET_NAME,
            Some(name) => name.as_str(),
        }
    }
}
