use crate::bucket::DEFAULT_FOLDER_NAME;

use serde_derive::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct LoadFileForm {
    #[schema(example = "/tmp/test_document.txt")]
    file_path: String,
    #[schema(example = "test_folder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_id: Option<String>,
}

impl LoadFileForm {
    pub fn get_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn get_bucket(&self) -> &str {
        match self.folder_id.as_ref() {
            None => DEFAULT_FOLDER_NAME,
            Some(name) => name.as_str(),
        }
    }
}
