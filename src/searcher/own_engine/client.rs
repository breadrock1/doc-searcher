use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::searcher::own_engine::context::OtherContext;
use crate::searcher::service_client::ServiceClient;
use crate::wrappers::bucket::{Bucket, BucketBuilder, BucketForm};
use crate::wrappers::cluster::{Cluster, ClusterBuilder};
use crate::wrappers::document::Document;
use crate::wrappers::search_params::SearchParams;

use actix_web::{web, HttpResponse, ResponseError};
use std::path::Path;

#[async_trait::async_trait]
impl ServiceClient for OtherContext {
    async fn get_all_clusters(&self) -> WebResponse<web::Json<Vec<Cluster>>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.clusters.read().await;
        let clusters_vec = map
            .values()
            .cloned()
            .collect::<Vec<Cluster>>();

        Ok(web::Json(clusters_vec))
    }

    async fn get_cluster(&self, cluster_id: &str) -> WebResponse<web::Json<Cluster>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.clusters.read().await;
        match map.get(cluster_id) {
            Some(cluster) => Ok(web::Json(cluster.clone())),
            None => {
                let msg = "failed to get cluster".to_string();
                Err(WebError::GetCluster(msg))
            }
        }
    }

    async fn create_cluster(&self, cluster_id: &str) -> HttpResponse {
        let cluster = ClusterBuilder::default()
            .ip("localhost".to_string())
            .heap_percent("70%".to_string())
            .ram_percent("70%".to_string())
            .cpu("7/10".to_string())
            .load_1m("anh value".to_string())
            .load_5m("anh value".to_string())
            .load_15m("anh value".to_string())
            .master("master".to_string())
            .name(cluster_id.to_string())
            .node_role(cluster_id.to_string())
            .build()
            .unwrap();

        let cxt = self.get_cxt().write().await;
        let mut map = cxt.clusters.write().await;
        match map.insert(cluster_id.to_string(), cluster) {
            None => SuccessfulResponse::ok_response("Ok"),
            Some(_) => SuccessfulResponse::ok_response("Updated"),
        }
    }

    async fn delete_cluster(&self, cluster_id: &str) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.clusters.write().await;
        match map.remove(cluster_id) {
            Some(_) => SuccessfulResponse::ok_response("Ok"),
            None => SuccessfulResponse::ok_response("Not exist cluster"),
        }
    }

    async fn get_all_buckets(&self) -> WebResponse<web::Json<Vec<Bucket>>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.buckets.read().await;
        let buckets_vec = map
            .values()
            .cloned()
            .collect::<Vec<Bucket>>();

        Ok(web::Json(buckets_vec))
    }

    async fn get_bucket(&self, _bucket_id: &str) -> WebResponse<web::Json<Bucket>> {
        Ok(web::Json(Bucket::default()))
    }

    async fn delete_bucket(&self, _bucket_id: &str) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn create_bucket(&self, _bucket_form: &BucketForm) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn check_duplication(&self, _bucket_id: &str, _document_id: &str) -> bool {
        false
    }

    async fn get_document(
        &self,
        _bucket_id: &str,
        _doc_id: &str,
    ) -> WebResponse<web::Json<Document>> {
        Ok(web::Json(Document::default()))
    }

    async fn create_document(&self, _doc_form: &Document) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn update_document(&self, _doc_form: &Document) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn delete_document(&self, _bucket_id: &str, _doc_id: &str) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn load_file_to_bucket(&self, _bucket_id: &str, _file_path: &str) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn search_all(&self, _s_params: &SearchParams) -> WebResponse<web::Json<Vec<Document>>> {
        Ok(web::Json(Vec::default()))
    }

    async fn search_bucket(
        &self,
        _bucket_id: &str,
        _s_params: &SearchParams,
    ) -> WebResponse<web::Json<Vec<Document>>> {
        Ok(web::Json(Vec::default()))
    }

    async fn similar_all(&self, _s_params: &SearchParams) -> WebResponse<web::Json<Vec<Document>>> {
        Ok(web::Json(Vec::default()))
    }

    async fn similar_bucket(
        &self,
        _bucket_id: &str,
        _s_params: &SearchParams,
    ) -> WebResponse<web::Json<Vec<Document>>> {
        Ok(web::Json(Vec::default()))
    }
}
