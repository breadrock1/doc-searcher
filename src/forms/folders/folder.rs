use crate::forms::TestExample;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

pub const DEFAULT_FOLDER_ID: &str = "common_folder";
pub const HISTORY_FOLDER_ID: &str = "history";
pub const ARTIFACTS_FOLDER_ID: &str = "artifacts";
pub const INFO_FOLDER_ID: &str = "info-folder";

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct Folder {
    #[schema(example = "yellow")]
    health: String,
    #[schema(example = "open")]
    status: String,
    #[schema(example = "test_folder", rename = "id")]
    #[serde(rename(serialize = "id"))]
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
    #[schema(example = "Test Folder Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
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
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name
    }
    pub fn update_docs_count(&mut self) {
        let def_val = "1".to_string();
        let count = self.get_docs_count().unwrap_or(&def_val);
        let count_int = i32::from_str(count.as_str()).unwrap_or(2);
        let deleted = self.get_docs_deleted().unwrap_or(&def_val);
        let deleted_int = i32::from_str(deleted.as_str()).unwrap_or(1);
        let result = count_int - deleted_int - 1;
        self.docs_count = Some(result.to_string());
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
            .name(Some("Test Folder Name".to_string()))
            .build()
            .unwrap()
    }
}
