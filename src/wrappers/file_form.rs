use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use serde_derive::Deserialize;

#[derive(MultipartForm)]
pub struct UploadFileForm {
    _file: TempFile,
}

#[derive(Deserialize)]
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
            None => "common_bucket",
            Some(name) => name.as_str(),
        }
    }
}

// pub async fn upload_file() {
//     const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10; // 10 MB
//     const MAX_FILE_COUNT: i32 = 1;
//
//     // reject malformed requests
//     match form.file.size {
//         0 => return HttpResponse::BadRequest().finish(),
//         length if length > MAX_FILE_SIZE.try_into().unwrap() => {
//             return HttpResponse::BadRequest()
//                 .body(format!("The uploaded file is too large. Maximum size is {} bytes.", MAX_FILE_SIZE));
//         },
//         _ => {}
//     };
//
//     let temp_file_path = form.file.file.path();
//     let file_name: &str = form
//         .file
//         .file_name
//         .as_ref()
//         .map(|m| m.as_ref())
//         .unwrap_or("null");
//
//     let mut file_path = PathBuf::from(&config.data_path);
//     file_path.push(&sanitize_filename::sanitize(&file_name));
//
//     match std::fs::rename(temp_file_path, file_path) {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
