use crate::errors::{JsonResponse, PaginateJsonResponse, SuccessfulResponse, WebError};

use wrappers::cluster::Cluster;
use wrappers::document::{Document, DocumentPreview, MoveDocumetsForm};
use wrappers::folder::{Folder, FolderForm};
use wrappers::s_params::SearchParams;
use wrappers::scroll::{AllScrollsForm, NextScrollForm};

use std::collections::HashMap;

pub type GroupedDocs = HashMap<String, Vec<Document>>;

#[async_trait::async_trait]
pub trait SearcherService {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>>;
    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster>;
    async fn create_cluster(&self, cluster_id: &str) -> Result<SuccessfulResponse, WebError>;
    async fn delete_cluster(&self, cluster_id: &str) -> Result<SuccessfulResponse, WebError>;

    async fn get_all_folders(&self) -> JsonResponse<Vec<Folder>>;
    async fn get_folder(&self, folder_id: &str) -> JsonResponse<Folder>;
    async fn get_folder_documents(
        &self,
        folder_id: &str,
        opt_params: Option<SearchParams>,
    ) -> PaginateJsonResponse<Vec<DocumentPreview>>;
    async fn delete_folder(&self, folder_id: &str) -> Result<SuccessfulResponse, WebError>;
    async fn create_folder(&self, folder_form: &FolderForm)
        -> Result<SuccessfulResponse, WebError>;

    async fn get_document(&self, folder_id: &str, doc_id: &str) -> JsonResponse<Document>;
    async fn create_document(&self, doc_form: &Document) -> Result<SuccessfulResponse, WebError>;
    async fn create_document_preview(
        &self,
        folder_id: &str,
        doc_form: &DocumentPreview,
    ) -> Result<SuccessfulResponse, WebError>;
    async fn update_document(&self, doc_form: &Document) -> Result<SuccessfulResponse, WebError>;
    async fn delete_document(
        &self,
        folder_id: &str,
        document_id: &str,
    ) -> Result<SuccessfulResponse, WebError>;
    async fn move_documents(
        &self,
        move_form: &MoveDocumetsForm,
    ) -> Result<SuccessfulResponse, WebError>;

    async fn launch_watcher_analysis(
        &self,
        document_ids: &[String],
    ) -> JsonResponse<Vec<DocumentPreview>>;

    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>>;
    async fn delete_pagination(
        &self,
        scroll_ids: &AllScrollsForm,
    ) -> Result<SuccessfulResponse, WebError>;
    async fn paginate(&self, curr_scroll: &NextScrollForm) -> PaginateJsonResponse<Vec<Document>>;

    async fn search(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>>;
    async fn search_tokens(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>>;
    async fn similarity(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>>;

    async fn upload_documents(
        &self,
        name: &str,
        path: &str,
    ) -> Result<Vec<DocumentPreview>, WebError>;

    #[cfg(feature = "enable-chunked")]
    async fn search_chunked(&self, s_params: &SearchParams) -> PaginateJsonResponse<GroupedDocs>;

    #[cfg(feature = "enable-chunked")]
    async fn search_chunked_tokens(
        &self,
        s_params: &SearchParams,
    ) -> PaginateJsonResponse<GroupedDocs>;

    #[cfg(feature = "enable-chunked")]
    async fn similarity_chunked(
        &self,
        s_params: &SearchParams,
    ) -> PaginateJsonResponse<GroupedDocs>;

    fn group_document_chunks(&self, documents: &[Document]) -> HashMap<String, Vec<Document>> {
        let mut grouped_documents: HashMap<String, Vec<Document>> = HashMap::new();
        documents.iter().for_each(|doc| {
            grouped_documents
                .entry(doc.get_doc_md5().to_owned())
                .or_default()
                .push(doc.to_owned())
        });

        grouped_documents
    }
}
