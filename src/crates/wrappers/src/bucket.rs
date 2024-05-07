use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use std::fmt::Display;

pub const DEFAULT_FOLDER_NAME: &str = "common_folder";

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct Folder {
    #[schema(example = "yellow")]
    pub health: String,

    #[schema(example = "open")]
    pub status: String,

    #[schema(example = "test_folder")]
    pub index: String,

    #[schema(example = "60qbF_yuTa2TXYd7soYb1A")]
    pub uuid: String,

    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<String>,

    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep: Option<String>,

    #[schema(example = "100")]
    #[serde(alias = "docs.count", skip_serializing_if = "Option::is_none")]
    pub docs_count: Option<String>,

    #[schema(example = "50")]
    #[serde(alias = "docs.deleted", skip_serializing_if = "Option::is_none")]
    pub docs_deleted: Option<String>,

    #[schema(example = "890.3kb")]
    #[serde(alias = "store.size", skip_serializing_if = "Option::is_none")]
    pub store_size: Option<String>,

    #[schema(example = "890.3kb")]
    #[serde(alias = "pri.store.size", skip_serializing_if = "Option::is_none")]
    pub pri_store_size: Option<String>,
}

impl Folder {
    pub fn builder() -> FolderBuilder {
        FolderBuilder::default()
    }
}

#[derive(Deserialize, IntoParams, Serialize, ToSchema)]
pub struct FolderForm {
    #[schema(example = "test_folder")]
    folder_id: String,
    #[schema(example = "true")]
    is_preview_schema: bool,
}

impl Display for FolderForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_data = &self.folder_id;
        write!(f, "{}", self_data.clone())
    }
}

impl Default for FolderForm {
    fn default() -> Self {
        FolderForm::new(DEFAULT_FOLDER_NAME, true)
    }
}

impl FolderForm {
    pub fn new(folder_id: &str, is_preview: bool) -> Self {
        FolderForm {
            folder_id: folder_id.to_string(),
            is_preview_schema: is_preview,
        }
    }

    pub fn builder() -> FolderBuilder {
        FolderBuilder::default()
    }

    pub fn get_id(&self) -> &str {
        self.folder_id.as_str()
    }

    pub fn is_preview(&self) -> bool {
        self.is_preview_schema
    }
}
