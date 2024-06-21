use crate::errors::{Successful, WebError, WebResult};
use crate::forms::clusters::cluster::Cluster;
use crate::forms::clusters::forms::CreateClusterForm;
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::{DocumentType, MoveDocsForm};
use crate::forms::folders::folder::Folder;
use crate::forms::folders::forms::{CreateFolderForm, DeleteFolderForm};
use crate::forms::pagination::forms::{DeletePaginationsForm, PaginateNextForm};
use crate::forms::pagination::pagination::Paginated;
use crate::forms::searcher::s_params::SearchParams;

use serde_json::Value;

pub(crate) type UploadedResult = Result<Vec<Document>, WebError>;
pub(crate) type PaginatedResult<T> = Result<Paginated<Vec<T>>, WebError>;

#[async_trait::async_trait]
pub trait ClusterService {
    async fn get_all_clusters(&self) -> WebResult<Vec<Cluster>>;
    async fn get_cluster(&self, id: &str) -> WebResult<Cluster>;
    async fn delete_cluster(&self, id: &str) -> WebResult<Successful>;
    async fn create_cluster(&self, id: &str, form: &CreateClusterForm) -> WebResult<Successful>;
}

#[async_trait::async_trait]
pub trait FolderService {
    async fn get_all_folders(&self, show_all: bool) -> WebResult<Vec<Folder>>;
    async fn get_folder(&self, folder_id: &str) -> WebResult<Folder>;
    async fn create_folder(&self, form: &CreateFolderForm) -> WebResult<Successful>;
    async fn delete_folder(&self, folder_id: &str, form: &DeleteFolderForm) -> WebResult<Successful>;
}

#[async_trait::async_trait]
pub trait DocumentService {
    async fn create_document(&self, folder_id: &str, doc: &Document, doc_type: &DocumentType) -> WebResult<Successful>;
    async fn get_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Document>;
    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Successful>;
    async fn move_documents(&self, folder_id: &str, form: &MoveDocsForm) -> WebResult<Successful>;
    async fn update_document(&self, folder_id: &str, value: &Value, doc_type: &DocumentType) -> WebResult<Successful>;
}

#[async_trait::async_trait]
pub trait WatcherService {
    async fn analyse_docs(&self, doc_ids: &[String], doc_type: &DocumentType) -> WebResult<Vec<Value>>;
    async fn upload_files(&self, name: &str, path: &str) -> UploadedResult;
}

#[async_trait::async_trait]
pub trait PaginatorService {
    async fn delete_session(&self, scroll_ids: &DeletePaginationsForm) -> WebResult<Successful>;
    async fn paginate(&self, curr_scroll: &PaginateNextForm, doc_type: &DocumentType) -> PaginatedResult<Value>;
}

#[async_trait::async_trait]
pub trait SearcherService {
    async fn search_records(&self, s_params: &SearchParams, doc_type: &DocumentType) -> PaginatedResult<Value>;
    async fn search_fulltext(&self, s_params: &SearchParams, doc_type: &DocumentType) -> PaginatedResult<Value>;
    async fn search_similar(&self, s_params: &SearchParams, doc_type: &DocumentType) -> PaginatedResult<Value>;
    async fn search_semantic(&self, s_params: &SearchParams, doc_type: &DocumentType) -> PaginatedResult<Value>;
}
