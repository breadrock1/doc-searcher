use crate::errors::WebResponse;
use crate::wrappers::bucket::{Bucket, BucketForm};
use crate::wrappers::cluster::Cluster;
use crate::wrappers::document::Document;
use crate::wrappers::search_params::SearchParameters;

use actix_web::{web, HttpResponse};

#[async_trait::async_trait]
pub trait ServiceClient {
    async fn get_all_clusters(&self) -> WebResponse<web::Json<Vec<Cluster>>>;
    async fn get_cluster(&self, cluster_id: &str) -> WebResponse<web::Json<Cluster>>;
    async fn create_cluster(&self, cluster_id: &str) -> HttpResponse;
    async fn delete_cluster(&self, cluster_id: &str) -> HttpResponse;

    async fn get_all_buckets(&self) -> WebResponse<web::Json<Vec<Bucket>>>;
    async fn get_bucket(&self, bucket_id: &str) -> WebResponse<web::Json<Bucket>>;
    async fn delete_bucket(&self, bucket_id: &str) -> HttpResponse;
    async fn create_bucket(&self, bucket_form: &BucketForm) -> HttpResponse;

    async fn get_document(&self, bucket_id: &str, doc_id: &str)
        -> WebResponse<web::Json<Document>>;
    async fn create_document(&self, doc_form: &Document) -> HttpResponse;
    async fn update_document(&self, doc_form: &Document) -> HttpResponse;
    async fn delete_document(&self, bucket_id: &str, doc_id: &str) -> HttpResponse;

    async fn search_from_all(
        &self,
        s_params: &SearchParameters,
    ) -> WebResponse<web::Json<Vec<Document>>>;
    async fn search_from_target(
        &self,
        bucket_id: &str,
        s_params: &SearchParameters,
    ) -> WebResponse<web::Json<Vec<Document>>>;

    async fn similar_from_all(
        &self,
        s_params: &SearchParameters,
    ) -> WebResponse<web::Json<Vec<Document>>>;
    async fn similar_from_target(
        &self,
        bucket_id: &str,
        s_params: &SearchParameters,
    ) -> WebResponse<web::Json<Vec<Document>>>;
}
