pub mod elastic;
pub mod endpoints;
pub mod forms;
pub mod models;

use crate::errors::{Successful, WebResult};
use crate::storage::forms::CreateFolderForm;
use crate::storage::forms::DocumentType;
use crate::storage::models::Document;
use crate::storage::models::Folder;

use serde_json::Value;

pub trait DocumentsTrait {
    fn get_folder_id(&self) -> &str;
    fn get_doc_id(&self) -> &str;
}

#[async_trait::async_trait]
pub trait FolderService {
    async fn get_all_folders(&self, show_all: bool) -> WebResult<Vec<Folder>>;
    async fn get_folder(&self, folder_id: &str) -> WebResult<Folder>;
    async fn create_folder(&self, form: &CreateFolderForm) -> WebResult<Successful>;
    async fn delete_folder(&self, folder_id: &str) -> WebResult<Successful>;
}

#[async_trait::async_trait]
pub trait DocumentService {
    async fn create_document(
        &self,
        folder_id: &str,
        doc: &Document,
        doc_type: &DocumentType,
    ) -> WebResult<Successful>;
    async fn get_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Document>;
    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Successful>;
    async fn update_document(
        &self,
        folder_id: &str,
        value: &Value,
        doc_type: &DocumentType,
    ) -> WebResult<Successful>;
}
