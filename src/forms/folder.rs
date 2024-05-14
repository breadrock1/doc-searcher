use crate::forms::TestExample;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::{IntoParams, ToSchema};

pub const DEFAULT_FOLDER_ID: &str = "common_folder";
pub const HISTORY_FOLDER_ID: &str = "history";

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct Folder {
    #[schema(example = "yellow")]
    health: String,
    #[schema(example = "open")]
    status: String,
    #[schema(example = "test_folder")]
    index: String,
    #[schema(example = "60qbF_yuTa2TXYd7soYb1A")]
    uuid: String,
    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pri: Option<String>,
    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    rep: Option<String>,
    #[schema(example = "100")]
    #[serde(alias = "docs.count", skip_serializing_if = "Option::is_none")]
    docs_count: Option<String>,
    #[schema(example = "50")]
    #[serde(alias = "docs.deleted", skip_serializing_if = "Option::is_none")]
    docs_deleted: Option<String>,
    #[schema(example = "890.3kb")]
    #[serde(alias = "store.size", skip_serializing_if = "Option::is_none")]
    store_size: Option<String>,
    #[schema(example = "890.3kb")]
    #[serde(alias = "pri.store.size", skip_serializing_if = "Option::is_none")]
    pri_store_size: Option<String>,
}

impl Folder {
    pub fn builder() -> FolderBuilder {
        FolderBuilder::default()
    }

    pub fn get_health(&self) -> &str {
        self.health.as_str()
    }

    pub fn get_status(&self) -> &str {
        self.status.as_str()
    }

    pub fn get_index(&self) -> &str {
        self.index.as_str()
    }

    pub fn get_uuid(&self) -> &str {
        self.uuid.as_str()
    }

    pub fn get_pri(&self) -> Option<&String> {
        self.pri.as_ref()
    }

    pub fn get_rep(&self) -> Option<&String> {
        self.rep.as_ref()
    }

    pub fn get_docs_count(&self) -> Option<&String> {
        self.docs_count.as_ref()
    }

    pub fn get_docs_deleted(&self) -> Option<&String> {
        self.docs_deleted.as_ref()
    }

    pub fn get_store_size(&self) -> Option<&String> {
        self.store_size.as_ref()
    }

    pub fn get_pri_store_size(&self) -> Option<&String> {
        self.pri_store_size.as_ref()
    }
}

impl TestExample<Folder> for Folder {
    fn test_example(_value: Option<&str>) -> Folder {
        Folder::builder()
            .health("yellow".to_string())
            .status("open".to_string())
            .index("test_folder".to_string())
            .uuid("fDdHOrwMSESM9OlhLsrMWQ".to_string())
            .pri(Some("1".to_string()))
            .rep(Some("1".to_string()))
            .docs_count(Some("0".to_string()))
            .docs_deleted(Some("2".to_string()))
            .store_size(Some("23812".to_string()))
            .pri_store_size(Some("23812".to_string()))
            .build()
            .unwrap()
    }
}

#[derive(Deserialize, Serialize)]
pub enum FolderType {
    #[serde(rename(deserialize = "document", serialize = "document",))]
    Document,
    #[serde(rename(deserialize = "vector", serialize = "vector",))]
    DocumentVector,
    #[serde(rename(deserialize = "preview", serialize = "preview",))]
    DocumentPreview,
}

#[derive(Deserialize, IntoParams, Serialize, ToSchema)]
pub struct FolderForm {
    #[schema(example = "test_folder")]
    folder_id: String,
    #[schema(example = "Test Folder")]
    folder_name: String,
    #[schema(example = "preview")]
    schema_type: FolderType,
}

impl Display for FolderForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_data = &self.folder_id;
        write!(f, "{}", self_data.clone())
    }
}

impl Default for FolderForm {
    fn default() -> Self {
        FolderForm::new(DEFAULT_FOLDER_ID, "Common Folder", FolderType::Document)
    }
}

impl FolderForm {
    pub fn new(folder_id: &str, folder_name: &str, schema_type: FolderType) -> Self {
        FolderForm {
            folder_id: folder_id.to_string(),
            folder_name: folder_name.to_string(),
            schema_type,
        }
    }

    pub fn builder() -> FolderBuilder {
        FolderBuilder::default()
    }

    pub fn get_id(&self) -> &str {
        self.folder_id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.folder_name.as_str()
    }

    pub fn get_schema(&self) -> &FolderType {
        &self.schema_type
    }
}
