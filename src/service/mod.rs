pub mod elastic;
pub mod own_engine;

use crate::errors::WebResponse;
use crate::wrappers::bucket::{Bucket, BucketForm};
use crate::wrappers::cluster::Cluster;
use crate::wrappers::document::Document;
use crate::wrappers::search_params::SearchParams;

use actix_files::NamedFile;
use actix_web::{web, HttpResponse};

pub type JsonResponse<T> = WebResponse<web::Json<T>>;

#[async_trait::async_trait]
pub trait ServiceClient {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>>;
    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster>;
    async fn create_cluster(&self, cluster_id: &str) -> HttpResponse;
    async fn delete_cluster(&self, cluster_id: &str) -> HttpResponse;

    async fn get_all_buckets(&self) -> JsonResponse<Vec<Bucket>>;
    async fn get_bucket(&self, bucket_id: &str) -> JsonResponse<Bucket>;
    async fn delete_bucket(&self, bucket_id: &str) -> HttpResponse;
    async fn create_bucket(&self, bucket_form: &BucketForm) -> HttpResponse;

    async fn check_duplication(&self, bucket_id: &str, document_id: &str) -> bool;

    async fn get_document(&self, bucket_id: &str, doc_id: &str) -> JsonResponse<Document>;
    async fn create_document(&self, doc_form: &Document) -> HttpResponse;
    async fn update_document(&self, doc_form: &Document) -> HttpResponse;
    async fn delete_document(&self, bucket_id: &str, doc_id: &str) -> HttpResponse;

    async fn load_file_to_bucket(&self, bucket_id: &str, file_path: &str) -> HttpResponse;
    async fn download_file(&self, bucket_id: &str, file_path: &str) -> Option<NamedFile>;

    async fn search_all(&self, s_params: &SearchParams) -> JsonResponse<Vec<Document>>;
    async fn search_bucket(
        &self,
        bucket_id: &str,
        s_params: &SearchParams,
    ) -> JsonResponse<Vec<Document>>;

    async fn similar_all(&self, s_params: &SearchParams) -> JsonResponse<Vec<Document>>;
    async fn similar_bucket(
        &self,
        bucket_id: &str,
        s_params: &SearchParams,
    ) -> JsonResponse<Vec<Document>>;

    async fn load_cache(&self, s_params: &SearchParams) -> Option<Vec<Document>>;
    async fn insert_cache(&self, s_params: &SearchParams, docs: Vec<Document>) -> Vec<Document>;
}
