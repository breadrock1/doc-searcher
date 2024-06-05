use crate::forms::documents::DocumentsTrait;
use crate::forms::folders::forms::{CreateFolderForm, FolderType};

use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

#[derive(Builder, Clone, Default, Deserialize, Serialize)]
pub struct InfoFolder {
    index: String,
    name: String,
    location: String,
    user_id: String,
    folder_type: FolderType,
    is_system: bool,
}

#[allow(dead_code)]
impl InfoFolder {
    pub fn builder() -> InfoFolderBuilder {
        InfoFolderBuilder::default()
    }
    pub fn get_id(&self) -> &str {
        self.index.as_str()
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_location(&self) -> &str {
        self.location.as_str()
    }
    pub fn get_type(&self) -> &FolderType {
        &self.folder_type
    }
    pub fn is_system(&self) -> bool {
        self.is_system
    }
}

impl DocumentsTrait for InfoFolder {
    fn get_folder_id(&self) -> &str {
        self.index.as_str()
    }

    fn get_doc_id(&self) -> &str {
        ""
    }

    fn set_folder_id(&mut self, folder_id: &str) {
        self.index = folder_id.to_string();
    }
}

impl From<&CreateFolderForm> for InfoFolder {
    fn from(value: &CreateFolderForm) -> Self {
        InfoFolder {
            index: value.get_id().to_string(),
            name: value.get_name().to_string(),
            location: value.get_location().to_string(),
            user_id: value.get_user().to_string(),
            folder_type: value.get_schema().to_owned(),
            is_system: value.is_system(),
        }
    }
}
