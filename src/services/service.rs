use crate::errors::{JsonResponse, PaginateResponse, WebError, WebResult};

use crate::forms::cluster::Cluster;
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::MoveDocumentsForm;
use crate::forms::folder::{Folder, FolderForm};
use crate::forms::pagination::{AllScrollsForm, NextScrollForm};
use crate::forms::preview::DocumentPreview;
use crate::forms::s_params::SearchParams;

pub(crate) type UploadedResult = Result<Vec<DocumentPreview>, WebError>;

#[async_trait::async_trait]
pub trait ClustersService {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>>;
    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster>;
    async fn create_cluster(&self, cluster_id: &str) -> WebResult;
    async fn delete_cluster(&self, cluster_id: &str) -> WebResult;
}

#[async_trait::async_trait]
pub trait FoldersService {
    async fn get_all_folders(&self) -> JsonResponse<Vec<Folder>>;
    async fn get_folder(&self, folder_id: &str) -> JsonResponse<Folder>;
    async fn get_folder_documents(
        &self,
        folder_id: &str,
        opt_params: Option<SearchParams>,
    ) -> PaginateResponse<Vec<DocumentPreview>>;
    async fn delete_folder(&self, folder_id: &str) -> WebResult;
    async fn create_folder(&self, folder_form: &FolderForm) -> WebResult;
}

#[async_trait::async_trait]
pub trait DocumentsService {
    async fn get_document(&self, folder_id: &str, doc_id: &str) -> JsonResponse<Document>;
    async fn create_document(&self, doc_form: &Document) -> WebResult;
    async fn create_document_preview(
        &self,
        folder_id: &str,
        doc_form: &DocumentPreview,
    ) -> WebResult;
    async fn update_document(&self, doc_form: &Document) -> WebResult;
    async fn delete_document(&self, folder_id: &str, document_id: &str) -> WebResult;
    async fn move_documents(&self, move_form: &MoveDocumentsForm) -> WebResult;
}

#[async_trait::async_trait]
pub trait WatcherService {
    async fn launch_analysis(&self, document_ids: &[String]) -> JsonResponse<Vec<DocumentPreview>>;
    async fn upload_files(&self, name: &str, path: &str) -> UploadedResult;
}

#[async_trait::async_trait]
pub trait PaginatorService {
    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>>;
    async fn delete_pagination(&self, scroll_ids: &AllScrollsForm) -> WebResult;
    async fn paginate(&self, curr_scroll: &NextScrollForm) -> PaginateResponse<Vec<Document>>;
}

#[async_trait::async_trait]
pub trait SearcherService {
    async fn search(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>>;
    async fn search_tokens(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>>;
    async fn similarity(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>>;
}
