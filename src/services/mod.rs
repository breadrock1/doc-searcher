pub mod cacher;
pub mod elastic;
pub mod init;
pub mod own_engine;

use crate::errors::{JsonResponse, PaginateJsonResponse};

use actix_files::NamedFile;
use actix_web::HttpResponse;

use redis::{FromRedisValue, ToRedisArgs};
use std::collections::HashMap;

use wrappers::bucket::{Folder, FolderForm};
use wrappers::cluster::Cluster;
use wrappers::document::{Document, DocumentPreview};
use wrappers::scroll::{AllScrolls, NextScroll};
use wrappers::search_params::SearchParams;

pub type GroupedDocs = HashMap<String, Vec<Document>>;

#[derive(Clone)]
pub struct CacherClient<D: CacherService> {
    pub service: D,
}

impl<D: CacherService> CacherClient<D> {
    pub fn new(service: D) -> Self {
        CacherClient { service }
    }
}

#[async_trait::async_trait]
pub trait CacherService {
    async fn insert<T, U>(&self, key: T, value: U) -> U
    where
        T: ToRedisArgs + Send + Sync,
        U: ToRedisArgs + Send + Sync;

    async fn load<T, U>(&self, key: T) -> Option<U>
    where
        T: ToRedisArgs + Send + Sync,
        U: FromRedisValue + Send + Sync;
}

#[async_trait::async_trait]
pub trait SearcherService {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>>;
    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster>;
    async fn create_cluster(&self, cluster_id: &str) -> HttpResponse;
    async fn delete_cluster(&self, cluster_id: &str) -> HttpResponse;

    async fn get_all_folders(&self) -> JsonResponse<Vec<Folder>>;
    async fn get_folder(&self, folder_id: &str) -> JsonResponse<Folder>;
    async fn get_folder_documents(
        &self,
        folder_id: &str,
        opt_params: Option<SearchParams>,
    ) -> PaginateJsonResponse<Vec<DocumentPreview>>;
    async fn delete_folder(&self, folder_id: &str) -> HttpResponse;
    async fn create_folder(&self, folder_form: &FolderForm) -> HttpResponse;

    async fn get_document(&self, folder_id: &str, doc_id: &str) -> JsonResponse<Document>;
    async fn create_document(&self, doc_form: &Document) -> HttpResponse;
    async fn create_document_preview(
        &self,
        folder_id: &str,
        doc_form: &DocumentPreview,
    ) -> HttpResponse;
    async fn update_document(&self, doc_form: &Document) -> HttpResponse;
    async fn delete_document(&self, folder_id: &str, document_id: &str) -> HttpResponse;
    async fn move_documents(
        &self,
        folder_id: &str,
        src_folder_id: &str,
        document_ids: &[String],
    ) -> HttpResponse;

    async fn load_file_to_bucket(&self, folder_id: &str, file_path: &str) -> HttpResponse;
    async fn download_file(&self, folder_id: &str, file_path: &str) -> Option<NamedFile>;
    async fn launch_watcher_analysis(
        &self,
        document_ids: &[String],
    ) -> JsonResponse<Vec<DocumentPreview>>;

    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>>;
    async fn delete_pagination_ids(&self, scroll_ids: &AllScrolls) -> HttpResponse;
    async fn next_pagination_result(
        &self,
        curr_scroll: &NextScroll,
    ) -> PaginateJsonResponse<Vec<Document>>;

    async fn search(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>>;
    async fn search_tokens(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>>;
    async fn similarity(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>>;

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
                .entry(doc.document_md5.to_owned())
                .or_default()
                .push(doc.to_owned())
        });

        grouped_documents
    }
}
