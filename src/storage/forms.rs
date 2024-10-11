use crate::errors::WebError;
use crate::storage::models::Document;
use crate::storage::models::DocumentPreview;
use crate::storage::models::DocumentVectors;

use derive_builder::{Builder};
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FolderType {
    #[default]
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "vectors")]
    Vectors,
    #[serde(rename = "info-folder")]
    InfoFolder,
}

#[derive(Builder, Deserialize, Serialize, Getters, CopyGetters, IntoParams, ToSchema)]
pub struct CreateFolderForm {
    #[getset(get = "pub")]
    #[schema(example = "test-folder")]
    folder_id: String,
    #[getset(get = "pub")]
    #[schema(example = "Test Folder")]
    folder_name: String,
    #[getset(get = "pub")]
    #[schema(example = "preview")]
    folder_type: FolderType,
    #[schema(example = false)]
    create_into_watcher: bool,
    #[getset(get = "pub")]
    #[schema(example = "/tmp")]
    location: String,
    #[getset(get = "pub")]
    #[schema(example = "admin")]
    user_id: String,
    #[getset(get_copy = "pub")]
    #[schema(example = false)]
    is_system: bool,
}

impl CreateFolderForm {
    pub fn builder() -> CreateFolderFormBuilder {
        CreateFolderFormBuilder::default()
    }
}

#[derive(Deserialize, Default, IntoParams, ToSchema)]
pub struct ShowAllFlag {
    show_all: Option<bool>,
}

impl ShowAllFlag {
    pub fn flag(&self) -> bool {
        self.show_all.unwrap_or(false)
    }
}

#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
pub enum DocumentType {
    #[default]
    #[serde(rename_all = "kebab-case")]
    Document,
    #[serde(rename_all = "kebab-case")]
    Preview,
    #[serde(rename_all = "kebab-case")]
    Vectors,
    #[serde(rename_all = "kebab-case")]
    GroupedVectors,
}

impl DocumentType {
    pub fn to_value(&self, document: &Document) -> Result<Value, WebError> {
        match self {
            DocumentType::Preview => serde_json::to_value(DocumentPreview::from(document)),
            DocumentType::Vectors => serde_json::to_value(DocumentVectors::from(document)),
            _ => serde_json::to_value(document),
        }
            .map_err(WebError::from)
    }
    pub fn is_vector_type(&self) -> bool {
        matches!(self, DocumentType::Vectors)
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
