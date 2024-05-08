use crate::errors::{JsonResponse, PaginateJsonResponse, SuccessfulResponse, WebError};
use crate::services::own_engine::context::OtherContext;
use crate::services::own_engine::helper;
use crate::services::SearcherService;

#[cfg(feature = "enable-chunked")]
use crate::services::GroupedDocs;

use wrappers::bucket::{Folder, FolderForm};
use wrappers::cluster::Cluster;
use wrappers::document::{Document, DocumentPreview};
use wrappers::scroll::{AllScrolls, NextScroll, PaginatedResult};
use wrappers::search_params::SearchParams;

use actix_files::NamedFile;
use actix_web::{web, HttpResponse, ResponseError};

use std::path::Path;

#[async_trait::async_trait]
impl SearcherService for OtherContext {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.clusters.read().await;
        let clusters_vec = map.values().cloned().collect::<Vec<Cluster>>();

        Ok(web::Json(clusters_vec))
    }

    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.clusters.read().await;
        match map.get(cluster_id) {
            Some(cluster) => Ok(web::Json(cluster.clone())),
            None => {
                log::warn!("Failed while getting cluster: {}", cluster_id);
                let msg = "failed to get cluster".to_string();
                Err(WebError::GetCluster(msg))
            }
        }
    }

    async fn create_cluster(&self, cluster_id: &str) -> HttpResponse {
        let cluster = Cluster::builder()
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
            Some(_) => SuccessfulResponse::ok_response("Updated"),
            None => {
                let msg = format!("Created {}", cluster_id);
                log::info!("New cluster has been created: {}", msg);
                SuccessfulResponse::ok_response(msg.as_str())
            }
        }
    }

    async fn delete_cluster(&self, cluster_id: &str) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.clusters.write().await;
        match map.remove(cluster_id) {
            Some(_) => SuccessfulResponse::ok_response("Ok"),
            None => {
                let msg = "Not exist cluster".to_string();
                log::warn!("Failed while deleting cluster: {}", msg.as_str());
                WebError::DeletingCluster(msg).error_response()
            }
        }
    }

    async fn get_all_folders(&self) -> JsonResponse<Vec<Folder>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.buckets.read().await;
        let buckets_vec = map.values().cloned().collect::<Vec<Folder>>();

        Ok(web::Json(buckets_vec))
    }

    async fn get_folder(&self, bucket_id: &str) -> JsonResponse<Folder> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.buckets.read().await;
        match map.get(bucket_id) {
            Some(bucket) => Ok(web::Json(bucket.clone())),
            None => {
                let msg = "Not exists".to_string();
                log::warn!("Failed while getting bucket {}", bucket_id);
                Err(WebError::GetBucket(msg))
            }
        }
    }

    async fn get_folder_documents(
        &self,
        _bucket_id: &str,
        _opt_params: Option<SearchParams>,
    ) -> PaginateJsonResponse<Vec<DocumentPreview>> {
        let documents_vec = Vec::default();
        Ok(web::Json(PaginatedResult::new(documents_vec)))
    }

    async fn delete_folder(&self, bucket_id: &str) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let uuid = bucket_id.to_string();
        let mut map = cxt.buckets.write().await;
        match map.remove(&uuid) {
            Some(_) => SuccessfulResponse::ok_response("Ok"),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while deleting bucket: {}", msg.as_str());
                WebError::DeleteBucket(msg).error_response()
            }
        }
    }

    async fn create_folder(&self, bucket_form: &FolderForm) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let uuid = bucket_form.get_id().to_string();
        let built_bucket = Folder::builder()
            .health("health".to_string())
            .status("status".to_string())
            .index(uuid.clone())
            .uuid(uuid.clone())
            .docs_count(Some("docs_count".to_string()))
            .store_size(Some("store_size".to_string()))
            .docs_deleted(Some("docs_deleted".to_string()))
            .pri_store_size(Some("pri_store_size".to_string()))
            .pri(None)
            .rep(None)
            .build();

        let mut map = cxt.buckets.write().await;
        match map.insert(uuid, built_bucket.unwrap()) {
            Some(bucket) => SuccessfulResponse::ok_response(bucket.uuid.as_str()),
            None => {
                let msg = format!("Created {}", bucket_form.get_id());
                log::warn!("New bucket has been created: {}", msg.as_str());
                SuccessfulResponse::ok_response(msg.as_str())
            }
        }
    }

    async fn get_document(&self, _bucket_id: &str, doc_id: &str) -> JsonResponse<Document> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        match map.get(doc_id) {
            Some(document) => Ok(web::Json(document.clone())),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while getting bucket: {}", msg.as_str());
                Err(WebError::GetDocument(msg))
            }
        }
    }

    async fn create_document(&self, doc_form: &Document) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.documents.write().await;
        match map.insert(doc_form.document_name.clone(), doc_form.clone()) {
            Some(_document) => SuccessfulResponse::ok_response("Ok"),
            None => {
                let msg = format!("Created {}", doc_form.document_name.as_str());
                log::warn!("Failed while creating document: {}", msg.as_str());
                SuccessfulResponse::ok_response(msg.as_str())
            }
        }
    }

    async fn create_document_preview(
        &self,
        _folder_id: &str,
        _doc_form: &DocumentPreview,
    ) -> HttpResponse {
        SuccessfulResponse::ok_response("Done")
    }

    async fn update_document(&self, doc_form: &Document) -> HttpResponse {
        self.create_document(doc_form).await
    }

    async fn delete_document(&self, _bucket_id: &str, doc_id: &str) -> HttpResponse {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.documents.write().await;
        match map.remove(doc_id) {
            Some(_) => SuccessfulResponse::ok_response("Ok"),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while deleting document: {}", msg.as_str());
                WebError::DeleteDocument(msg).error_response()
            }
        }
    }

    async fn move_documents(&self, _folder_id: &str, _src_folder_id: &str, _document_ids: &[String]) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn load_file_to_bucket(&self, bucket_id: &str, file_path: &str) -> HttpResponse {
        let path = Path::new(file_path);
        let file_data_vec = loader::load_passed_file_by_path(bucket_id, path);
        if file_data_vec.is_empty() {
            let msg = "Failed to load file".to_string();
            log::warn!("Failed load file to bucket `{}`: {}", bucket_id, msg);
            return WebError::LoadFileFailed(msg).error_response();
        }

        SuccessfulResponse::ok_response("Ok")
    }

    async fn download_file(&self, _bucket_id: &str, file_path: &str) -> Option<NamedFile> {
        match NamedFile::open_async(file_path).await {
            Ok(named_file) => Some(named_file),
            Err(err) => {
                log::error!("Failed while opening async streaming: {}", err);
                None
            }
        }
    }

    async fn launch_watcher_analysis(
        &self,
        _document_ids: &[String],
    ) -> JsonResponse<Vec<DocumentPreview>> {
        Ok(web::Json(Vec::default()))
    }

    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>> {
        let def_vals: Vec<String> = Vec::default();
        Ok(web::Json(def_vals))
    }
    async fn delete_pagination_ids(&self, _ids: &AllScrolls) -> HttpResponse {
        SuccessfulResponse::ok_response("Ok")
    }

    async fn next_pagination_result(
        &self,
        _curr_scroll: &NextScroll,
    ) -> PaginateJsonResponse<Vec<Document>> {
        Ok(web::Json(PaginatedResult::new_with_id(
            Vec::default(),
            "id".to_string(),
        )))
    }

    async fn search(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let bucket_id = s_params.buckets.clone().unwrap_or("*".to_string());
        let documents_vec = helper::filter_founded_documents(map, bucket_id.as_str(), s_params);

        Ok(web::Json(PaginatedResult::new(documents_vec)))
    }

    async fn search_tokens(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let bucket_id = s_params.buckets.clone().unwrap_or("*".to_string());
        let documents_vec = helper::filter_founded_documents(map, bucket_id.as_str(), s_params);

        Ok(web::Json(PaginatedResult::new(documents_vec)))
    }

    async fn similarity(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let bucket_id = s_params.buckets.clone().unwrap_or("*".to_string());
        let documents_vec = map
            .values()
            .filter(|doc| doc.folder_id.eq(bucket_id.as_str()))
            .filter(|document| {
                hasher::compare_ssdeep_hashes(
                    s_params.query.as_str(),
                    document.document_ssdeep.as_str(),
                )
            })
            .cloned()
            .collect::<Vec<Document>>();

        Ok(web::Json(PaginatedResult::new(documents_vec)))
    }

    #[cfg(feature = "enable-chunked")]
    async fn search_chunked(&self, s_params: &SearchParams) -> PaginateJsonResponse<GroupedDocs> {
        match self.search(s_params).await {
            Ok(docs) => {
                let documents = docs.0.get_founded();
                let grouped = self.group_document_chunks(documents);
                Ok(web::Json(PaginatedResult::new(grouped)))
            }
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(err)
            }
        }
    }

    #[cfg(feature = "enable-chunked")]
    async fn search_chunked_tokens(
        &self,
        s_params: &SearchParams,
    ) -> PaginateJsonResponse<GroupedDocs> {
        match self.search_tokens(s_params).await {
            Ok(docs) => {
                let documents = docs.0.get_founded();
                let grouped = self.group_document_chunks(documents);
                Ok(web::Json(PaginatedResult::new(grouped)))
            }
            Err(err) => {
                log::error!("Failed while searching documents tokens: {}", err);
                Err(err)
            }
        }
    }

    #[cfg(feature = "enable-chunked")]
    async fn similarity_chunked(
        &self,
        s_params: &SearchParams,
    ) -> PaginateJsonResponse<GroupedDocs> {
        match self.similarity(s_params).await {
            Ok(docs) => {
                let documents = docs.0.get_founded();
                let grouped = self.group_document_chunks(documents);
                Ok(web::Json(PaginatedResult::new(grouped)))
            }
            Err(err) => {
                log::error!("Failed while searching similar documents: {}", err);
                Err(err)
            }
        }
    }
}
