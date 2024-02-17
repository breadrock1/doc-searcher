use crate::errors::{SuccessfulResponse, WebError, WebResponse};
use crate::service::own_engine::context::OtherContext;
use crate::service::ServiceClient;

use cacher::values::VecCacherDocuments;
use cacher::AnyCacherService;
use wrappers::bucket::{Bucket, BucketBuilder, BucketForm};
use wrappers::cluster::{Cluster, ClusterBuilder};
use wrappers::document::Document;
use wrappers::search_params::SearchParams;

use actix_files::NamedFile;
use actix_web::{web, HttpResponse, ResponseError};
use std::path::Path;

#[async_trait::async_trait]
impl ServiceClient for OtherContext {
    async fn get_all_clusters(&self) -> WebResponse<web::Json<Vec<Cluster>>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.clusters.read().await;
        let clusters_vec = map.values().cloned().collect::<Vec<Cluster>>();

        Ok(web::Json(clusters_vec))
    }

    async fn get_cluster(&self, cluster_id: &str) -> WebResponse<web::Json<Cluster>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.clusters.read().await;
        match map.get(cluster_id) {
            Some(cluster) => Ok(web::Json(cluster.clone())),
            None => {
                println!("Failed while getting cluster: {}", cluster_id);
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
        let buckets_vec = map.values().cloned().collect::<Vec<Bucket>>();

        Ok(web::Json(buckets_vec))
    }

    async fn get_bucket(&self, bucket_id: &str) -> WebResponse<web::Json<Bucket>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.buckets.read().await;
        match map.get(bucket_id) {
            Some(bucket) => Ok(web::Json(bucket.clone())),
            None => {
                println!("Failed while getting bucket {}", bucket_id);
                let msg = "failed to get bucket".to_string();
                Err(WebError::GetBucket(msg))
            }
        }
    }

    async fn delete_bucket(&self, bucket_id: &str) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let uuid = bucket_id.to_string();
        let mut map = cxt.buckets.write().await;
        match map.remove(&uuid) {
            Some(_) => SuccessfulResponse::ok_response("Ok"),
            None => SuccessfulResponse::ok_response("Empty buckets"),
        }
    }

    async fn create_bucket(&self, bucket_form: &BucketForm) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let uuid = bucket_form.get_name().to_string();
        let built_bucket = BucketBuilder::default()
            .health("health".to_string())
            .status("status".to_string())
            .index(uuid.clone())
            .uuid(uuid.clone())
            .docs_count("docs_count".to_string())
            .docs_deleted("docs_deleted".to_string())
            .store_size("store_size".to_string())
            .pri_store_size("pri_store_size".to_string())
            .pri(None)
            .rep(None)
            .build();

        let mut map = cxt.buckets.write().await;
        match map.insert(uuid, built_bucket.unwrap()) {
            None => SuccessfulResponse::ok_response("Ok"),
            Some(bucket) => SuccessfulResponse::ok_response(bucket.uuid.as_str()),
        }
    }

    async fn check_duplication(&self, _bucket_id: &str, document_id: &str) -> bool {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        map.contains_key(document_id)
    }

    async fn get_document(
        &self,
        _bucket_id: &str,
        doc_id: &str,
    ) -> WebResponse<web::Json<Document>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        match map.get(doc_id) {
            Some(document) => Ok(web::Json(document.clone())),
            None => {
                let msg = "failed to get document".to_string();
                Err(WebError::GetDocument(msg))
            }
        }
    }

    async fn create_document(&self, doc_form: &Document) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.documents.write().await;
        match map.insert(doc_form.document_name.clone(), doc_form.clone()) {
            None => SuccessfulResponse::ok_response("Ok"),
            Some(document) => SuccessfulResponse::ok_response(document.document_name.as_str()),
        }
    }

    async fn update_document(&self, doc_form: &Document) -> HttpResponse {
        self.create_document(doc_form).await
    }

    async fn delete_document(&self, _bucket_id: &str, doc_id: &str) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.documents.write().await;
        match map.remove(doc_id) {
            None => SuccessfulResponse::ok_response("Not existing document"),
            Some(document) => SuccessfulResponse::ok_response(document.document_name.as_str()),
        }
    }

    async fn load_file_to_bucket(&self, bucket_id: &str, file_path: &str) -> HttpResponse {
        let path = Path::new(file_path);
        let file_data_vec = loader::load_passed_file_by_path(path);
        if file_data_vec.is_empty() {
            let msg = "failed to load file".to_string();
            println!("Failed load file to bucket `{}`: {}", bucket_id, msg);
            return WebError::LoadFileFailed(msg).error_response();
        }

        SuccessfulResponse::ok_response("Ok")
    }

    async fn download_file(&self, _bucket_id: &str, file_path: &str) -> Option<NamedFile> {
        match actix_files::NamedFile::open_async(file_path).await {
            Ok(named_file) => Some(named_file),
            Err(err) => {
                println!("Failed while opening async streaming: {}", err);
                None
            }
        }
    }

    async fn search_all(&self, s_params: &SearchParams) -> WebResponse<web::Json<Vec<Document>>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let documents_vec = map
            .values()
            .filter(|document| document.content.contains(&s_params.query))
            .cloned()
            .collect::<Vec<Document>>();

        Ok(web::Json(documents_vec))
    }

    async fn search_bucket(
        &self,
        bucket_id: &str,
        s_params: &SearchParams,
    ) -> WebResponse<web::Json<Vec<Document>>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let documents_vec = map
            .values()
            .filter(|document| document.bucket_uuid.eq(bucket_id))
            .filter(|document| document.content.contains(&s_params.query))
            .cloned()
            .collect::<Vec<Document>>();

        Ok(web::Json(documents_vec))
    }

    async fn similar_all(&self, s_params: &SearchParams) -> WebResponse<web::Json<Vec<Document>>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let documents_vec = map
            .values()
            .filter(|document| {
                hasher::compare_ssdeep_hashes(
                    s_params.query.as_str(),
                    document.document_ssdeep_hash.as_str(),
                )
            })
            .cloned()
            .collect::<Vec<Document>>();

        Ok(web::Json(documents_vec))
    }

    async fn similar_bucket(
        &self,
        bucket_id: &str,
        s_params: &SearchParams,
    ) -> WebResponse<web::Json<Vec<Document>>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let documents_vec = map
            .values()
            .filter(|document| document.bucket_uuid.eq(bucket_id))
            .filter(|document| {
                hasher::compare_ssdeep_hashes(
                    s_params.query.as_str(),
                    document.document_ssdeep_hash.as_str(),
                )
            })
            .cloned()
            .collect::<Vec<Document>>();

        Ok(web::Json(documents_vec))
    }

    async fn load_cache(&self, s_params: &SearchParams) -> Option<Vec<Document>> {
        let cacher = self.get_cacher().read().await;
        let documents_opt = cacher.get_documents(s_params).await;
        let documenst = documents_opt?.get_documents().to_owned();

        Some(documenst)
    }

    async fn insert_cache(&self, s_params: &SearchParams, docs: Vec<Document>) -> Vec<Document> {
        let cacher = self.get_cacher().read().await;
        let vec_cacher_docs = VecCacherDocuments::from(docs);
        cacher
            .set_documents(s_params, vec_cacher_docs)
            .await
            .get_documents()
            .to_owned()
    }
}
