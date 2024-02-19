pub mod elastic;
pub mod init;
pub mod own_engine;
pub mod init;

use crate::errors::WebResponse;

use wrappers::bucket::{Bucket, BucketForm};
use wrappers::cluster::Cluster;
use wrappers::document::Document;
use wrappers::search_params::SearchParams;

use actix_files::NamedFile;
use actix_web::{web, HttpResponse};

use std::collections::HashMap;

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

    async fn get_document(&self, bucket_id: &str, doc_id: &str) -> JsonResponse<Document>;
    async fn create_document(&self, doc_form: &Document) -> HttpResponse;
    async fn update_document(&self, doc_form: &Document) -> HttpResponse;
    async fn delete_document(&self, bucket_id: &str, doc_id: &str) -> HttpResponse;

    async fn load_file_to_bucket(&self, bucket_id: &str, file_path: &str) -> HttpResponse;
    async fn download_file(&self, bucket_id: &str, file_path: &str) -> Option<NamedFile>;

    async fn search(&self, s_params: &SearchParams)
        -> JsonResponse<HashMap<String, Vec<Document>>>;
    async fn search_tokens(&self, s_params: &SearchParams) -> JsonResponse<Vec<Document>>;
    async fn similarity(&self, s_params: &SearchParams) -> JsonResponse<Vec<Document>>;

    async fn load_cache(&self, s_params: &SearchParams) -> Option<HashMap<String, Vec<Document>>>;
    async fn insert_cache(&self, s_params: &SearchParams, docs: Vec<Document>) -> Vec<Document>;
}
