use crate::forms::TestExample;

use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
pub enum FolderType {
    #[default]
    #[serde(rename(deserialize = "document", serialize = "document",))]
    Document,
    #[serde(rename(deserialize = "vectors", serialize = "vectors",))]
    Vectors,
    #[serde(rename(deserialize = "info-folder", serialize = "info-folder",))]
    InfoFolder,
}

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct CreateFolderForm {
    #[schema(example = "test-folder")]
    folder_id: String,
    #[schema(example = "Test Folder")]
    folder_name: String,
    #[schema(example = "preview")]
    folder_type: FolderType,
    #[schema(example = false)]
    create_into_watcher: bool,
    #[schema(example = "/tmp")]
    location: String,
    #[schema(example = "aTfbs823bfs8a")]
    user_id: String,
    #[schema(example = false)]
    is_system: bool,
}

impl CreateFolderForm {
    pub fn get_id(&self) -> &str {
        self.folder_id.as_str()
    }
    pub fn get_name(&self) -> &str {
        self.folder_name.as_str()
    } 
    pub fn get_schema(&self) -> &FolderType {
        &self.folder_type
    }
    pub fn create_into_watcher(&self) -> bool {
        self.create_into_watcher
    }
    pub fn get_location(&self) -> &str {
        self.location.as_str()
    }
    pub fn get_user(&self) -> &str {
        self.user_id.as_str()
    }
    pub fn is_system(&self) -> bool {
        self.is_system
    }
}

impl TestExample<CreateFolderForm> for CreateFolderForm {
    fn test_example(_value: Option<&str>) -> CreateFolderForm {
        CreateFolderForm {
            folder_id: "test-folder".to_string(),
            folder_name: "Test Folder".to_string(),
            folder_type: FolderType::Document,
            create_into_watcher: false,
            location: "/tmp".to_string(),
            user_id: "aTfbs823bfs8a".to_string(),
            is_system: false,
        }
    }
}

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct DeleteFolderForm {
    #[schema(example = false)]
    delete_into_watcher: bool,
}

impl DeleteFolderForm {
    pub fn delete_into_watcher(&self) -> bool {
        self.delete_into_watcher
    }
}

impl TestExample<DeleteFolderForm> for DeleteFolderForm {
    fn test_example(_value: Option<&str>) -> DeleteFolderForm {
        DeleteFolderForm {
            delete_into_watcher: false,
        }
    }
}

#[derive(Deserialize, Default, IntoParams, ToSchema)]
pub struct ShowAllFlag {
    show_all: Option<bool>,
}

impl ShowAllFlag {
    pub fn flag(&self) -> bool {
        self.show_all.unwrap_or_else(|| false)
    }
}
