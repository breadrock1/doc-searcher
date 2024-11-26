use crate::storage::models::FolderType;

use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Default, Deserialize, IntoParams, ToSchema)]
pub struct FolderTypeQuery {
    folder_type: Option<FolderType>,
}

impl FolderTypeQuery {
    pub fn folder_type(&self) -> FolderType {
        self.folder_type.clone().unwrap_or_default()
    }
}

#[derive(Builder, Deserialize, Serialize, Getters, CopyGetters, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct CreateFolderForm {
    #[schema(example = "test-folder")]
    folder_id: String,

    #[schema(example = "Test Folder")]
    folder_name: String,

    #[schema(example = "preview")]
    folder_type: FolderType,

    #[getset(skip)]
    #[schema(example = false)]
    create_into_watcher: bool,

    #[schema(example = "/tmp")]
    location: String,

    #[schema(example = "admin")]
    user_id: String,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[schema(example = false)]
    is_system: bool,
}

impl CreateFolderForm {
    pub fn builder() -> CreateFolderFormBuilder {
        CreateFolderFormBuilder::default()
    }
}

#[derive(Default, Deserialize, IntoParams, ToSchema)]
pub struct ShowAllFlag {
    show_all: Option<bool>,
}

impl ShowAllFlag {
    pub fn show_all(&self) -> bool {
        self.show_all.unwrap_or(false)
    }
}

#[derive(
    Builder, Debug, Deserialize, Serialize, Getters, CopyGetters, Setters, IntoParams, ToSchema,
)]
#[getset(get = "pub")]
pub struct RetrieveParams {
    #[schema(example = "Any folder or document name or path")]
    query: Option<String>,

    #[schema(example = "txt")]
    document_extension: Option<String>,

    #[getset(skip)]
    #[schema(example = 0)]
    document_size_to: Option<i64>,

    #[getset(skip)]
    #[schema(example = 0)]
    document_size_from: Option<i64>,

    #[getset(skip)]
    #[schema(example = "2024-04-26T11:14:55Z")]
    created_date_to: Option<String>,

    #[getset(skip)]
    #[schema(example = "2024-04-02T13:51:32Z")]
    created_date_from: Option<String>,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[schema(example = 10)]
    result_size: i64,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[schema(example = 0)]
    result_offset: i64,

    #[getset(skip)]
    #[getset(get_copy = "pub", set = "pub")]
    #[schema(example = true)]
    is_show_all: bool,
}

impl Default for RetrieveParams {
    fn default() -> Self {
        RetrieveParams::builder()
            .query(None)
            .document_extension(None)
            .created_date_to(None)
            .created_date_from(None)
            .document_size_to(None)
            .document_size_from(None)
            .result_size(25)
            .result_offset(0)
            .is_show_all(false)
            .build()
            .unwrap()
    }
}

impl RetrieveParams {
    pub fn builder() -> RetrieveParamsBuilder {
        RetrieveParamsBuilder::default()
    }

    pub fn document_size(&self) -> (Option<i64>, Option<i64>) {
        (self.document_size_from, self.document_size_to)
    }

    pub fn document_dates(&self) -> (Option<String>, Option<String>) {
        (self.created_date_from.clone(), self.created_date_to.clone())
    }
}
