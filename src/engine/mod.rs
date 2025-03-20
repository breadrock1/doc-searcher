pub mod elastic;
pub mod error;
pub mod form;
pub mod model;

use serde_json::Value;

use crate::engine::error::{PaginatedResult, SearcherResult, StorageResult};
use crate::engine::form::{CreateFolderForm, DocumentType};
use crate::engine::form::{DeleteScrollsForm, ScrollNextForm};
use crate::engine::form::{FulltextParams, RetrieveParams, SemanticParams};
use crate::engine::model::{Document, Folder, FolderType};
use crate::errors::Successful;

#[async_trait::async_trait]
pub trait FolderService {
    async fn get_folders(&self, show_all: bool) -> StorageResult<Vec<Folder>>;
    async fn get_folder(&self, folder_id: &str) -> StorageResult<Folder>;
    async fn create_folder(&self, form: &CreateFolderForm) -> StorageResult<Successful>;
    async fn delete_folder(&self, folder_id: &str) -> StorageResult<Successful>;
}

#[async_trait::async_trait]
pub trait DocumentService {
    async fn get_documents(
        &self,
        folder_id: &str,
        folder_type: &FolderType,
        params: &RetrieveParams,
    ) -> StorageResult<Vec<Value>>;
    async fn get_document(
        &self,
        folder_id: &str,
        doc_id: &str,
        folder_type: &FolderType,
    ) -> StorageResult<Value>;
    async fn create_document(
        &self,
        folder_id: &str,
        doc: &Document,
        folder_type: &FolderType,
    ) -> StorageResult<Successful>;
    async fn update_document(
        &self,
        folder_id: &str,
        doc: &Value,
        folder_type: &FolderType,
    ) -> StorageResult<Successful>;
    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> StorageResult<Successful>;
}

#[async_trait::async_trait]
pub trait SearcherService {
    async fn search_fulltext(
        &self,
        params: &FulltextParams,
        return_as: &DocumentType,
    ) -> PaginatedResult<Value>;
    async fn search_semantic(&self, params: &SemanticParams) -> PaginatedResult<Value>;
}

#[async_trait::async_trait]
pub trait PaginatorService {
    async fn delete_session(&self, form: &DeleteScrollsForm) -> SearcherResult<Successful>;
    async fn paginate(
        &self,
        form: &ScrollNextForm,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value>;
}
