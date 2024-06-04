use crate::forms::TestExample;
use crate::forms::documents::forms::DocumentType;

use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct CreateFolderForm {
    #[schema(example = "test-folder")]
    folder_id: String,
    #[schema(example = "Test Folder")]
    folder_name: String,
    #[schema(example = "preview")]
    folder_schema_type: DocumentType,
    #[schema(example = false)]
    create_into_watcher: bool,
}

impl CreateFolderForm {
    pub fn get_id(&self) -> &str {
        self.folder_id.as_str()
    }
    pub fn get_name(&self) -> &str {
        self.folder_name.as_str()
    } 
    pub fn get_schema(&self) -> &DocumentType {
        &self.folder_schema_type
    }
    pub fn create_into_watcher(&self) -> bool {
        self.create_into_watcher
    }
}

impl TestExample<CreateFolderForm> for CreateFolderForm {
    fn test_example(_value: Option<&str>) -> CreateFolderForm {
        CreateFolderForm {
            folder_id: "test-folder".to_string(),
            folder_name: "Test Folder".to_string(),
            folder_schema_type: DocumentType::Document,
            create_into_watcher: false,
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
