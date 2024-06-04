use crate::errors::{Successful, WebError, WebResult};

use crate::forms::clusters::cluster::Cluster;
use crate::forms::documents::document::Document;
use crate::forms::documents::embeddings::DocumentVectors;
use crate::forms::documents::forms::MoveDocsForm;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::similar::DocumentSimilar;
use crate::forms::folders::folder::Folder;
use crate::forms::folders::forms::{CreateFolderForm, DeleteFolderForm};
use crate::forms::pagination::forms::{DeletePaginationsForm, PaginateNextForm};
use crate::forms::pagination::pagination::Paginated;
use crate::forms::searcher::s_params::SearchParams;

use serde_json::Value;

pub(crate) type UploadedResult = Result<Vec<Document>, WebError>;
pub(crate) type PaginatedResult<T> = Result<Paginated<Vec<T>>, WebError>;

#[async_trait::async_trait]
pub trait ClustersService {
    async fn get_all_clusters(&self) -> WebResult<Vec<Cluster>>;
    async fn get_cluster(&self, cluster_id: &str) -> WebResult<Cluster>;
    async fn create_cluster(&self, cluster_id: &str) -> WebResult<Successful>;
    async fn delete_cluster(&self, cluster_id: &str) -> WebResult<Successful>;
}

#[async_trait::async_trait]
pub trait FoldersService {
    async fn get_all_folders(&self) -> WebResult<Vec<Folder>>;
    async fn get_folder(&self, folder_id: &str) -> WebResult<Folder>;
    async fn create_folder(&self, form: &CreateFolderForm) -> WebResult<Successful>;
    async fn delete_folder(&self, folder_id: &str, form: &DeleteFolderForm) -> WebResult<Successful>;
}

#[async_trait::async_trait]
pub trait DocumentsService {
    async fn create_document(&self, doc: &Document) -> WebResult<Successful>;
    async fn get_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Document>;
    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> WebResult<Successful>;
    async fn update_document(&self, folder_id: &str, doc_id: &str, value: &Value) -> WebResult<Successful>;
}

#[async_trait::async_trait]
pub trait WatcherService {
    async fn move_documents(&self, folder_id: &str, form: &MoveDocsForm) -> WebResult<Successful>;
    async fn analyse_docs(&self, doc_ids: &[String]) -> WebResult<Vec<Document>>;
    async fn upload_files(&self, name: &str, path: &str) -> UploadedResult;
}

#[async_trait::async_trait]
pub trait PaginatorService {
    async fn delete_session(&self, scroll_ids: &DeletePaginationsForm) -> WebResult<Successful>;
    async fn paginate(&self, curr_scroll: &PaginateNextForm) -> PaginatedResult<Document>;
}

#[async_trait::async_trait]
pub trait SearcherService {
    async fn search_previews(&self, s_params: &SearchParams) -> PaginatedResult<DocumentPreview>;
    async fn search_fulltext(&self, s_params: &SearchParams) -> PaginatedResult<Document>;
    async fn search_semantic(&self, s_params: &SearchParams) -> PaginatedResult<DocumentVectors>;
    async fn search_similar(&self, s_params: &SearchParams) -> PaginatedResult<DocumentSimilar>;
}
