use crate::errors::{JsonResponse, PaginateResponse, SuccessfulResponse, WebError};
use crate::services::own_engine::context::OtherContext;
use crate::services::own_engine::helper;
use crate::services::searcher;

use crate::forms::cluster::Cluster;
use crate::forms::document::{Document, DocumentPreview, MoveDocumetsForm};
use crate::forms::folder::{Folder, FolderForm};
use crate::forms::s_params::SearchParams;
use crate::forms::scroll::{AllScrollsForm, NextScrollForm, Paginated};

use actix_web::web;

#[async_trait::async_trait]
impl searcher::ClustersService for OtherContext {
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
    async fn create_cluster(&self, cluster_id: &str) -> Result<SuccessfulResponse, WebError> {
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
            Some(_) => Ok(SuccessfulResponse::success("Updated")),
            None => {
                let msg = format!("Created {}", cluster_id);
                log::info!("New cluster has been created: {}", msg);
                Ok(SuccessfulResponse::success(msg.as_str()))
            }
        }
    }
    async fn delete_cluster(&self, cluster_id: &str) -> Result<SuccessfulResponse, WebError> {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.clusters.write().await;
        match map.remove(cluster_id) {
            Some(_) => Ok(SuccessfulResponse::success("Ok")),
            None => {
                let msg = "Not exist cluster".to_string();
                log::warn!("Failed while deleting cluster: {}", msg.as_str());
                Err(WebError::DeleteCluster(msg))
            }
        }
    }
}

#[async_trait::async_trait]
impl searcher::FoldersService for OtherContext {
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
                Err(WebError::GetFolder(msg))
            }
        }
    }
    async fn get_folder_documents(
        &self,
        _bucket_id: &str,
        _opt_params: Option<SearchParams>,
    ) -> PaginateResponse<Vec<DocumentPreview>> {
        let documents_vec = Vec::default();
        Ok(web::Json(Paginated::new(documents_vec)))
    }
    async fn delete_folder(&self, bucket_id: &str) -> Result<SuccessfulResponse, WebError> {
        let cxt = self.get_cxt().write().await;
        let uuid = bucket_id.to_string();
        let mut map = cxt.buckets.write().await;
        match map.remove(&uuid) {
            Some(_) => Ok(SuccessfulResponse::success("Ok")),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while deleting bucket: {}", msg.as_str());
                Err(WebError::DeleteFolder(msg))
            }
        }
    }
    async fn create_folder(
        &self,
        bucket_form: &FolderForm,
    ) -> Result<SuccessfulResponse, WebError> {
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
            Some(bucket) => Ok(SuccessfulResponse::success(bucket.get_uuid())),
            None => {
                let msg = format!("Created {}", bucket_form.get_id());
                log::warn!("New bucket has been created: {}", msg.as_str());
                Ok(SuccessfulResponse::success(msg.as_str()))
            }
        }
    }
}

#[async_trait::async_trait]
impl searcher::DocumentsService for OtherContext {
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
    async fn create_document(&self, doc_form: &Document) -> Result<SuccessfulResponse, WebError> {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.documents.write().await;
        let doc_name = doc_form.get_doc_name();
        match map.insert(doc_name.to_string(), doc_form.clone()) {
            Some(_document) => Ok(SuccessfulResponse::success("Ok")),
            None => {
                let msg = format!("Created {}", doc_name);
                log::warn!("Failed while creating document: {}", msg.as_str());
                Ok(SuccessfulResponse::success(msg.as_str()))
            }
        }
    }
    async fn create_document_preview(
        &self,
        _folder_id: &str,
        _doc_form: &DocumentPreview,
    ) -> Result<SuccessfulResponse, WebError> {
        Ok(SuccessfulResponse::success("Ok"))
    }
    async fn update_document(&self, doc_form: &Document) -> Result<SuccessfulResponse, WebError> {
        self.create_document(doc_form).await
    }
    async fn delete_document(
        &self,
        _bucket_id: &str,
        doc_id: &str,
    ) -> Result<SuccessfulResponse, WebError> {
        let cxt = self.get_cxt().write().await;
        let mut map = cxt.documents.write().await;
        match map.remove(doc_id) {
            Some(_) => Ok(SuccessfulResponse::success("Ok")),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while deleting document: {}", msg.as_str());
                Err(WebError::DeleteDocument(msg))
            }
        }
    }
    async fn move_documents(
        &self,
        _move_form: &MoveDocumetsForm,
    ) -> Result<SuccessfulResponse, WebError> {
        Ok(SuccessfulResponse::success("Ok"))
    }
}

#[async_trait::async_trait]
impl searcher::WatcherService for OtherContext {
    async fn launch_analysis(
        &self,
        _document_ids: &[String],
    ) -> JsonResponse<Vec<DocumentPreview>> {
        Ok(web::Json(Vec::default()))
    }
    async fn upload_files(
        &self,
        _name: &str,
        _path: &str,
    ) -> Result<Vec<DocumentPreview>, WebError> {
        Ok(Vec::default())
    }
}

#[async_trait::async_trait]
impl searcher::PaginatorService for OtherContext {
    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>> {
        let def_vals: Vec<String> = Vec::default();
        Ok(web::Json(def_vals))
    }
    async fn delete_pagination(
        &self,
        _ids: &AllScrollsForm,
    ) -> Result<SuccessfulResponse, WebError> {
        Ok(SuccessfulResponse::success("Ok"))
    }
    async fn paginate(&self, _curr_scroll: &NextScrollForm) -> PaginateResponse<Vec<Document>> {
        Ok(web::Json(Paginated::new_with_id(
            Vec::default(),
            "id".to_string(),
        )))
    }
}

#[async_trait::async_trait]
impl searcher::SearcherService for OtherContext {
    async fn search(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let folder_id = s_params.get_folders(true);
        let documents_vec = helper::filter_founded_documents(map, folder_id.as_str(), s_params);

        Ok(web::Json(Paginated::new(documents_vec)))
    }
    async fn search_tokens(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let folder_id = s_params.get_folders(true);
        let documents_vec = helper::filter_founded_documents(map, folder_id.as_str(), s_params);

        Ok(web::Json(Paginated::new(documents_vec)))
    }
    async fn similarity(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.documents.read().await;
        let folder_id = s_params.get_folders(true);
        let documents_vec = map
            .values()
            .filter(|doc| doc.get_folder_id().eq(folder_id.as_str()))
            .filter(|document| {
                hasher::compare_ssdeep_hashes(s_params.get_query(), document.get_doc_ssdeep())
            })
            .cloned()
            .collect::<Vec<Document>>();

        Ok(web::Json(Paginated::new(documents_vec)))
    }
}
