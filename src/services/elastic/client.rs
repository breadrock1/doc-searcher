use crate::errors::{JsonResponse, PaginateJsonResponse, SuccessfulResponse, WebError};
use crate::services::elastic::helper;
use crate::services::elastic::{context, watcher};
use crate::services::searcher::SearcherService;

#[cfg(feature = "enable-chunked")]
use crate::services::searcher::GroupedDocs;

use wrappers::cluster::Cluster;
use wrappers::document::{Document, DocumentPreview, MoveDocumetsForm};
use wrappers::folder::{Folder, FolderForm, HISTORY_FOLDER_ID};
use wrappers::s_params::SearchParams;
use wrappers::scroll::{AllScrollsForm, NextScrollForm, PaginatedResult};

use actix_web::web;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use elasticsearch::{ClearScrollParts, IndexParts, ScrollParts};
use serde_json::{json, Value};

#[async_trait::async_trait]
impl SearcherService for context::ElasticContext {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>> {
        let elastic = self.get_cxt().read().await;
        let response = helper::get_all_clusters(&elastic).await?;
        match response.json::<Vec<Cluster>>().await {
            Ok(clusters) => Ok(web::Json(clusters)),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }
    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster> {
        let elastic = self.get_cxt().read().await;
        let response = helper::get_all_clusters(&elastic).await?;
        match response.json::<Vec<Cluster>>().await {
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
            Ok(clusters) => {
                let founded_cluster = clusters
                    .iter()
                    .filter(|cluster| cluster.get_name().eq(cluster_id))
                    .map(|cluster| cluster.to_owned())
                    .collect::<Vec<Cluster>>();

                match founded_cluster.first() {
                    Some(value) => Ok(web::Json(value.to_owned())),
                    None => {
                        let msg = format!("There is no cluster with passed name: {}", cluster_id);
                        log::error!("{}", msg.as_str());
                        Err(WebError::GetCluster(msg))
                    }
                }
            }
        }
    }
    async fn create_cluster(&self, _cluster_id: &str) -> Result<SuccessfulResponse, WebError> {
        let msg = "This functionality does not implemented yet!";
        log::warn!("{}", msg);
        Err(WebError::CreateCluster(msg.to_string()))
    }
    async fn delete_cluster(&self, cluster_id: &str) -> Result<SuccessfulResponse, WebError> {
        let elastic = self.get_cxt().read().await;
        let json_data: Value = json!({
            "transient": {
                "cluster.routing.allocation.exclude._ip": cluster_id
            }
        });

        let json_str = serde_json::to_string(&json_data).unwrap();
        let body = json_str.as_bytes();
        let response = elastic
            .send(
                Method::Put,
                "/_cluster/settings",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(body),
                None,
            )
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }

    async fn get_all_folders(&self) -> JsonResponse<Vec<Folder>> {
        let elastic = self.get_cxt().read().await;
        let response = elastic
            .send(
                Method::Get,
                "/_cat/indices?format=json",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        if !response.status_code().is_success() {
            return Err(helper::extract_exception(response).await);
        }

        match response.json::<Vec<Folder>>().await {
            Ok(folders) => Ok(web::Json(folders)),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }
    async fn get_folder(&self, folder_id: &str) -> JsonResponse<Folder> {
        let elastic = self.get_cxt().read().await;
        let target_url = format!("/{}/_stats", folder_id);
        let response = elastic
            .send(
                Method::Get,
                target_url.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        if !response.status_code().is_success() {
            return Err(helper::extract_exception(response).await);
        }

        let json_value = response.json::<Value>().await?;
        match helper::extract_folder_stats(&json_value) {
            Ok(folders) => Ok(web::Json(folders)),
            Err(err) => {
                log::error!("Failed while extracting folders stats: {}", err);
                Err(err)
            }
        }
    }
    async fn get_folder_documents(
        &self,
        folder_id: &str,
        opt_params: Option<SearchParams>,
    ) -> PaginateJsonResponse<Vec<DocumentPreview>> {
        let elastic = self.get_cxt().read().await;
        let s_params = opt_params.unwrap_or_else(SearchParams::default);

        // TODO: Implement storing data to unrecognized folder
        if folder_id.eq("unrecognized") {
            let cxt_opts = self.get_options().as_ref();
            return match watcher::get_unrecognized_docs(cxt_opts, &s_params).await {
                Err(err) => Err(err),
                Ok(documents) => Ok(web::Json(PaginatedResult::new(documents))),
            };
        }

        let body = helper::build_match_all_query(&s_params);
        match helper::search_documents_preview(&elastic, &s_params, &body, &[folder_id]).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(err)
            }
        }
    }
    async fn delete_folder(&self, folder_id: &str) -> Result<SuccessfulResponse, WebError> {
        let cxt_opts = self.get_options().as_ref();
        let result = watcher::remove_folder(cxt_opts, folder_id).await?;
        if !result.is_success() {
            let msg = format!("Failed to remove folder: {}", folder_id);
            log::error!("{}", msg.as_str());
            return Err(WebError::DeleteFolder(msg));
        }

        let elastic = self.get_cxt().read().await;
        let response = elastic
            .send(
                Method::Delete,
                folder_id,
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn create_folder(
        &self,
        folder_form: &FolderForm,
    ) -> Result<SuccessfulResponse, WebError> {
        let cxt_opts = self.get_options().as_ref();
        let result = watcher::create_folder(cxt_opts, folder_form).await?;
        if !result.is_success() {
            let msg = format!("Failed to create folder: {}", folder_form.get_id());
            log::error!("{}", msg.as_str());
            return Err(WebError::CreateFolder(msg));
        }

        let elastic = self.get_cxt().read().await;
        let folder_id = folder_form.get_id();
        let folder_schema = helper::create_folder_schema(folder_form.is_preview());
        let response = elastic
            .index(IndexParts::Index(folder_id))
            .body(&json!({folder_id: folder_schema}))
            .send()
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }

    async fn get_document(&self, folder_id: &str, doc_id: &str) -> JsonResponse<Document> {
        let elastic = self.get_cxt().read().await;
        let s_doc_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response = elastic
            .send(
                Method::Get,
                s_doc_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        let document = helper::extract_document(response).await?;
        Ok(web::Json(document))
    }
    async fn create_document(&self, doc_form: &Document) -> Result<SuccessfulResponse, WebError> {
        let doc_id = doc_form.get_doc_id();
        let folder_id = doc_form.get_folder_id();
        let elastic = self.get_cxt().read().await;
        let is_exists = helper::check_duplication(&elastic, folder_id, doc_id).await?;
        if is_exists {
            let msg = format!("Passed document: {} already exists", doc_id);
            return Err(WebError::CreateDocument(msg));
        }

        helper::store_document(&elastic, doc_form, folder_id).await
    }
    async fn create_document_preview(
        &self,
        folder_id: &str,
        doc_form: &DocumentPreview,
    ) -> Result<SuccessfulResponse, WebError> {
        // TODO: Impled for Document and DocumentPreview into create_document()
        let elastic = self.get_cxt().read().await;
        helper::store_doc_preview(&elastic, doc_form, folder_id).await
    }
    async fn update_document(&self, doc_form: &Document) -> Result<SuccessfulResponse, WebError> {
        // TODO: Impled for Document and DocumentPreview
        let elastic = self.get_cxt().read().await;
        let document_json = serde_json::to_value(doc_form).map_err(WebError::from)?;

        let doc_id = doc_form.get_doc_id();
        let folder_id = doc_form.get_folder_id();
        let s_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response = elastic
            .send(
                Method::Put,
                s_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(document_json.to_string().as_bytes()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn delete_document(
        &self,
        folder_id: &str,
        doc_id: &str,
    ) -> Result<SuccessfulResponse, WebError> {
        let elastic = self.get_cxt().read().await;
        let s_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response = elastic
            .send(
                Method::Delete,
                s_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn move_documents(
        &self,
        move_form: &MoveDocumetsForm,
    ) -> Result<SuccessfulResponse, WebError> {
        let opts = self.get_options();
        let move_result = watcher::move_docs_to_folder(opts.as_ref(), move_form)
            .await
            .map_err(WebError::from)?;

        let src_folder_id = move_form.get_src_folder_id();
        if !move_result.is_success() {
            let msg = format!("Failed while moving documents from: {}", src_folder_id);
            log::error!("{}", msg.as_str());
            return Err(WebError::MoveDocuments(msg));
        }

        let mut collected_errors = Vec::default();
        for document_id in move_form.get_document_ids() {
            let result = self.get_document(src_folder_id, document_id).await;
            if result.is_err() {
                let err = result.err().unwrap();
                log::error!("Failed while move document: {}", err.to_string());
                collected_errors.push(document_id.to_owned());
                continue;
            }

            let document = result.unwrap().0;

            let result = self.delete_document(src_folder_id, document_id).await;
            if result.is_err() {
                let err = result.err().unwrap();
                log::error!("Failed while move document: {}", err.to_string());
                collected_errors.push(document_id.to_owned());
                continue;
            }

            let result = self.create_document(&document).await;
            if result.is_err() {
                let err = result.err().unwrap();
                log::error!("Failed while move document: {}", err.to_string());
                collected_errors.push(document_id.to_owned());
            }
        }

        if collected_errors.len() > 0 {
            let collected_docs = collected_errors.join(", ");
            let msg = format!("Failed while move document: {}", collected_docs);
            return Err(WebError::MoveDocuments(msg));
        }

        Ok(SuccessfulResponse::success("Ok"))
    }
    async fn launch_watcher_analysis(
        &self,
        document_ids: &[String],
    ) -> JsonResponse<Vec<DocumentPreview>> {
        let cxt_opts = self.get_options().as_ref();
        let analysed_docs = watcher::launch_analysis(cxt_opts, document_ids).await?;
        for doc_preview in analysed_docs.iter() {
            let folder_id = doc_preview.get_folder_id();
            let _ = self
                .create_document_preview(HISTORY_FOLDER_ID, doc_preview)
                .await;
            let _ = self.create_document_preview(folder_id, doc_preview).await;
        }

        Ok(web::Json(analysed_docs))
    }

    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .send(
                Method::Post,
                "/_search/scroll=10m",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        match response_result {
            Ok(response) => {
                log::info!("Pag Ids: {}", response.text().await.unwrap());
                let def_vals: Vec<String> = Vec::default();
                Ok(web::Json(def_vals))
            }
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(WebError::SearchError(err.to_string()))
            }
        }
    }
    async fn delete_pagination(
        &self,
        ids: &AllScrollsForm,
    ) -> Result<SuccessfulResponse, WebError> {
        let elastic = self.get_cxt().read().await;
        let response = elastic
            .clear_scroll(ClearScrollParts::ScrollId(&ids.as_slice()))
            .send()
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn paginate(&self, scroll_form: &NextScrollForm) -> PaginateJsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .scroll(ScrollParts::ScrollId(scroll_form.get_scroll_id()))
            .pretty(true)
            .send()
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            log::error!("Failed to get next pagination: {}", err.to_string());
            return Err(WebError::PaginationError(err.to_string()));
        }

        let response = response_result.unwrap();
        Ok(web::Json(helper::parse_search_result(response).await))
    }

    async fn search(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_query(s_params);
        let folders = s_params.get_folders(false);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, s_params, &body_value, indexes.as_slice()).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(err)
            }
        }
    }
    async fn search_tokens(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_query(s_params);
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, s_params, &body_value, indexes.as_slice()).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching documents tokens: {}", err);
                Err(err)
            }
        }
    }
    async fn similarity(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_similar_query(s_params);
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, s_params, &body_value, indexes.as_slice()).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching similar documents: {}", err);
                Err(err)
            }
        }
    }

    async fn upload_documents(
        &self,
        name: &str,
        path: &str,
    ) -> Result<Vec<DocumentPreview>, WebError> {
        let cxt_opts = self.get_options().as_ref();
        watcher::translate_multipart_form(cxt_opts, name.to_string(), path.to_string()).await
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
