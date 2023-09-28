use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DocumentJson {
    doc_name: String,
    doc_path: String,
    doc_ext: String,
}

impl DocumentJson {
    pub fn new(name: &str, path: &str, ext: &str) -> Self {
        DocumentJson {
            doc_name: name.to_string(),
            doc_path: path.to_string(),
            doc_ext: ext.to_string(),
        }
    }
}
