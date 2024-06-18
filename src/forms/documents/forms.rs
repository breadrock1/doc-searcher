use crate::errors::WebError;
use crate::forms::TestExample;
use crate::forms::documents::document::Document;
use crate::forms::documents::embeddings::DocumentVectors;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::similar::DocumentSimilar;

use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
pub enum DocumentType {
    #[default]
    #[serde(rename(deserialize = "document", serialize = "document",))]
    Document,
    #[serde(rename(deserialize = "preview", serialize = "preview",))]
    Preview,
    #[serde(rename(deserialize = "vectors", serialize = "vectors",))]
    Vectors,
    #[serde(rename(deserialize = "similar", serialize = "similar",))]
    Similar,
}

impl DocumentType {
    pub fn to_value(&self, document: &Document) -> Result<Value, WebError> {
        match self {
            DocumentType::Preview => serde_json::to_value(DocumentPreview::from(document)),
            DocumentType::Vectors => serde_json::to_value(DocumentVectors::from(document)),
            DocumentType::Similar => serde_json::to_value(DocumentSimilar::from(document)),
            _ => serde_json::to_value(document)
        }
        .map_err(WebError::from)
    }
    pub fn is_vector_type(&self) -> bool {
        matches!(self, DocumentType::Vectors)
    }
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct MoveDocsForm {
    folder_id: String,
    location: String,
    document_ids: Vec<String>,
    is_watcher_enabled: bool,
}

impl MoveDocsForm {
    pub fn builder() -> MoveDocsFormBuilder {
        MoveDocsFormBuilder::default()
    }
    pub fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }
    pub fn get_location(&self) -> &str {
        self.location.as_str()
    }
    pub fn get_doc_ids(&self) -> &[String] {
        self.document_ids.as_slice()
    }
    pub fn use_watcher(&self) -> bool {
        self.is_watcher_enabled
    }
}

impl TestExample<MoveDocsForm> for MoveDocsForm {
    fn test_example(_value: Option<&str>) -> MoveDocsForm {
        MoveDocsForm::builder()
            .folder_id("test_folder".to_string())
            .location("target_folder".to_string())
            .document_ids(vec!["98ac9896be35f47fb8442580cd9839b4".to_string()])
            .is_watcher_enabled(false)
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct DeleteDocsForm {
    folder_id: String,
    document_ids: Vec<String>,
}

impl DeleteDocsForm {
    pub fn builder() -> DeleteDocsFormBuilder {
        DeleteDocsFormBuilder::default()
    }
    pub fn get_doc_ids(&self) -> &Vec<String> {
        self.document_ids.as_ref()
    }
    pub fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }
}

impl TestExample<DeleteDocsForm> for DeleteDocsForm {
    fn test_example(_value: Option<&str>) -> DeleteDocsForm {
        DeleteDocsForm::builder()
            .folder_id("test_folder".to_string())
            .document_ids(vec![
                "98ac9896be35f47fb8442580cd9839b4".to_string()
            ])
            .build()
            .unwrap()
    }
}

#[derive(Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct AnalyseDocsForm {
    document_ids: Vec<String>,
}

impl AnalyseDocsForm {
    pub fn get_doc_ids(&self) -> &[String] {
        self.document_ids.as_slice()
    }
}

impl TestExample<AnalyseDocsForm> for AnalyseDocsForm {
    fn test_example(_value: Option<&str>) -> AnalyseDocsForm {
        AnalyseDocsForm {
            document_ids: vec!["98ac9896be35f47fb8442580cd9839b4".to_string()],
        }
    }
}

#[derive(Deserialize, Default, IntoParams, ToSchema)]
pub struct DocTypeQuery {
    document_type: Option<DocumentType>,
}

impl DocTypeQuery {
    pub fn get_type(&self) -> DocumentType {
        self.document_type.clone().unwrap_or(DocumentType::Document)
    }
}
