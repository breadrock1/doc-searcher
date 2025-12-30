use doc_search_core::domain::storage::models::{LargeDocument, LargeDocumentBuilder};
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[allow(unused_imports)]
use serde_json::json;

use crate::server::httpserver::api::v1::form::Metadata;
use crate::server::ServerError;

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreateDocumentForm {
    #[schema(example = "test-document.docx")]
    file_name: String,
    #[schema(example = "./test-document.docx")]
    file_path: String,
    #[schema(example = 1024)]
    file_size: u32,
    #[schema(example = 1750957115)]
    created_at: i64,
    #[schema(example = 1750957115)]
    modified_at: i64,
    #[schema(example = "There is some content data")]
    content: String,
    metadata: Option<Metadata>,
}

impl TryFrom<CreateDocumentForm> for LargeDocument {
    type Error = ServerError;

    fn try_from(form: CreateDocumentForm) -> Result<Self, Self::Error> {
        let meta = match form.metadata {
            Some(data) => data.try_into().ok(),
            None => None,
        };

        LargeDocumentBuilder::default()
            .file_name(form.file_name)
            .file_path(form.file_path)
            .file_size(form.file_size)
            .content(form.content)
            .created_at(form.created_at)
            .modified_at(form.modified_at)
            .metadata(meta)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct UpdateDocumentForm {
    #[schema(example = "test-document.docx")]
    file_name: String,
    #[schema(example = "./test-document.docx")]
    file_path: String,
    #[schema(example = 1024)]
    file_size: u32,
    #[schema(example = 1750957115)]
    created_at: i64,
    #[schema(nullable, example = "There is some content data")]
    content: String,
}

impl TryFrom<UpdateDocumentForm> for LargeDocument {
    type Error = ServerError;

    fn try_from(form: UpdateDocumentForm) -> Result<Self, Self::Error> {
        let modified_dt = chrono::Utc::now().timestamp();
        LargeDocumentBuilder::default()
            .file_name(form.file_name)
            .file_path(form.file_path)
            .file_size(form.file_size)
            .content(form.content)
            .created_at(form.created_at)
            .modified_at(modified_dt)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
    }
}
