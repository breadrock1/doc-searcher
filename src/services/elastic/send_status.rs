#[derive(Default)]
pub struct SendDocumentStatus {
    is_success: bool,
    file_path: String,
}

impl SendDocumentStatus {
    pub fn new(status: bool, file_path: &str) -> Self {
        SendDocumentStatus {
            is_success: status,
            file_path: file_path.to_string(),
        }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }

    pub fn get_path(&self) -> String {
        self.file_path.clone()
    }
}
