use crate::forms::TestExample;

use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct MoveDocumentsForm {
    document_ids: Vec<String>,
    location: String,
    src_folder_id: String,
}

impl MoveDocumentsForm {
    pub fn builder() -> MoveDocumentsFormBuilder {
        MoveDocumentsFormBuilder::default()
    }

    pub fn get_folder_id(&self) -> &str {
        self.location.as_str()
    }

    pub fn get_src_folder_id(&self) -> &str {
        self.src_folder_id.as_str()
    }

    pub fn get_document_ids(&self) -> &[String] {
        self.document_ids.as_slice()
    }
}

impl TestExample<MoveDocumentsForm> for MoveDocumentsForm {
    fn test_example(_value: Option<&str>) -> MoveDocumentsForm {
        MoveDocumentsForm::builder()
            .location("Test Folder".to_string())
            .src_folder_id("unrecognized".to_string())
            .document_ids(vec!["98ac9896be35f47fb8442580cd9839b4".to_string()])
            .build()
            .unwrap()
    }
}

#[derive(Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct AnalyseDocumentsForm {
    pub document_ids: Vec<String>,
}

impl TestExample<AnalyseDocumentsForm> for AnalyseDocumentsForm {
    fn test_example(_value: Option<&str>) -> AnalyseDocumentsForm {
        AnalyseDocumentsForm {
            document_ids: vec!["98ac9896be35f47fb8442580cd9839b4".to_string()],
        }
    }
}
