use crate::forms::documents::document::Document;
use crate::forms::documents::DocumentsTrait;

use derive_builder::Builder;
use serde_derive::Serialize;

#[derive(Builder, Default, Clone, Serialize)]
pub struct DocumentSimilar {
    document: Document,
}

impl DocumentSimilar {
    pub fn new(document: Document) -> Self {
        DocumentSimilar { document }
    }
    pub fn builder() -> DocumentSimilarBuilder {
        DocumentSimilarBuilder::default()
    }
    pub fn get_query_fields() -> Vec<String> {
        vec!["content".to_string(), "document_ssdeep".to_string()]
    }
    pub fn get_document(&self) -> Document {
        self.document.to_owned()
    }
}

impl DocumentsTrait for DocumentSimilar {
    fn get_folder_id(&self) -> &str {
        self.document.get_folder_id()
    }

    fn get_doc_id(&self) -> &str {
        self.document.get_doc_ssdeep()
    }
    fn set_folder_id(&mut self, folder_id: &str) {
        self.document.set_folder_id(folder_id)
    }
}

impl From<&Document> for DocumentSimilar {
    fn from(value: &Document) -> Self {
        DocumentSimilar::builder()
            .document(value.to_owned())
            .build()
            .unwrap()
    }
}
