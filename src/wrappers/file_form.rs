use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use serde_derive::{Deserialize, Serialize};

#[derive(MultipartForm)]
pub struct UploadFileForm {
    file: TempFile,
}

#[derive(Deserialize, Serialize)]
pub struct LoadFileForm {
    file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<String>,
}

impl LoadFileForm {
    pub fn get_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn get_bucket(&self) -> Option<&String> {
        self.bucket_name.as_ref()
    }
}
