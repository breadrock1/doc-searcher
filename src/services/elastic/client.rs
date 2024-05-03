use crate::errors::{SuccessfulResponse, WebError};
use crate::services::elastic::{context, watcher};
use crate::services::elastic::helper;
use crate::services::{JsonResponse, PaginateJsonResponse, SearcherService};

#[cfg(feature = "enable-chunked")]
use crate::services::GroupedDocs;

use hasher::{gen_hash, HashType};
use wrappers::bucket::DEFAULT_FOLDER_NAME;
use wrappers::bucket::{Folder, FolderForm};
use wrappers::cluster::Cluster;
use wrappers::document::{Document, DocumentPreview};
use wrappers::scroll::{AllScrolls, NextScroll, PaginatedResult};
use wrappers::search_params::SearchParams;

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use elasticsearch::{ClearScrollParts, IndexParts, ScrollParts};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::{json, Value};

#[async_trait::async_trait]
impl SearcherService for context::ElasticContext {
    async fn get_all_clusters(&self) -> JsonResponse<Vec<Cluster>> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .send(
                Method::Get,
                "/_cat/nodes",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            log::error!("Failed while getting clusters: {}", err);
            return Err(WebError::GetCluster(err.to_string()));
        }

        let response = response_result.unwrap();
        let resp_json = response.json::<Value>().await?;
        match serde_json::from_value::<Vec<Cluster>>(resp_json) {
            Ok(clusters) => Ok(web::Json(clusters)),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }

    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .send(
                Method::Get,
                "/_cat/nodes",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            log::error!("Failed while getting cluster {}: {}", cluster_id, err);
            return Err(WebError::DeletingCluster(err.to_string()));
        }

        let response = response_result.unwrap();
        let resp_json = response.json::<Value>().await?;
        match serde_json::from_value::<Vec<Cluster>>(resp_json) {
            Ok(clusters) => {
                let founded_cluster = clusters
                    .iter()
                    .filter(|cluster| cluster.name.eq(cluster_id))
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
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }

    async fn create_cluster(&self, _cluster_id: &str) -> HttpResponse {
        let msg = "This functionality does not implemented yet!";
        log::warn!("{}", msg);
        WebError::CreateCluster(msg.to_string()).error_response()
    }

    async fn delete_cluster(&self, cluster_id: &str) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let json_data: Value = json!({
            "transient": {
                "cluster.routing.allocation.exclude._ip": cluster_id
            }
        });

        let body = json_data.as_str();
        if body.is_none() {
            let msg = "Json body is None".to_string();
            log::error!("Failed while building json body: {}", msg);
            return WebError::DeletingCluster(msg).error_response();
        }

        let body = body.unwrap().as_bytes();
        let response_result = elastic
            .send(
                Method::Put,
                "/_cluster/settings",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(body),
                None,
            )
            .await;

        match response_result {
            Ok(_) => SuccessfulResponse::ok_response("Ok"),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                WebError::DeletingCluster(err.to_string()).error_response()
            }
        }
    }

    async fn get_all_folders(&self) -> JsonResponse<Vec<Folder>> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .send(
                Method::Get,
                "/_cat/indices?format=json",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            log::error!("Failed while getting buckets: {}", err);
            return Err(WebError::from(err));
        }

        let response = response_result.unwrap();
        let resp_json = response.json::<Value>().await?;
        match serde_json::from_value::<Vec<Folder>>(resp_json) {
            Ok(buckets) => Ok(web::Json(buckets)),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }

    async fn get_folder(&self, bucket_id: &str) -> JsonResponse<Folder> {
        let elastic = self.get_cxt().read().await;
        let bucket_name = format!("/{}/_stats", bucket_id);
        let response_result = elastic
            .send(
                Method::Get,
                bucket_name.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            log::error!("Returned failed response from elastic: {}", err);
            return Err(WebError::from(err));
        }

        let response = response_result.unwrap();
        let json_value = response.json::<Value>().await;
        if json_value.is_err() {
            let err = json_value.err().unwrap();
            log::error!("Failed while parsing bucket {}: {}", bucket_id, err);
            return Err(WebError::from(err));
        }

        match helper::extract_bucket_stats(&json_value.unwrap()) {
            Ok(bucket) => Ok(web::Json(bucket)),
            Err(err) => {
                log::error!("Failed while extracting buckets stats: {}", err);
                Err(err)
            }
        }
    }

    async fn get_folder_documents(
        &self,
        bucket_id: &str,
        opt_params: Option<SearchParams>,
    ) -> PaginateJsonResponse<Vec<DocumentPreview>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_match_all_query();

        let s_params = opt_params.unwrap_or_else(|| SearchParams {
            result_size: 1000,
            ..Default::default()
        });
        
        if bucket_id.eq("unrecognized") {
            let cxt_opts = self.get_options().as_ref();
            return match watcher::get_unrecognized_documents(cxt_opts, &s_params).await {
                Err(err) => Err(err),
                Ok(documents) => {
                    Ok(web::Json(PaginatedResult::new(documents.0)))
                }
            };
        }

        match helper::search_documents_preview(&elastic, &[bucket_id], &body_value, &s_params).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(err)
            }
        }
    }

    async fn delete_folder(&self, bucket_id: &str) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .send(
                Method::Delete,
                bucket_id,
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        match response_result {
            Ok(_) => SuccessfulResponse::ok_response("Ok"),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                WebError::DeleteBucket(err.to_string()).error_response()
            }
        }
    }

    async fn create_folder(&self, bucket_form: &FolderForm) -> HttpResponse {
        // TODO: Implement creating another Schemas for history using enum
        let elastic = self.get_cxt().read().await;
        let bucket_name = bucket_form.get_name();
        let hasher_res = gen_hash(HashType::MD5, bucket_name.as_bytes());
        let binding = hasher_res.unwrap();
        let id_str = binding.get_hash_data();
        let bucket_schema: Value = serde_json::from_str(helper::create_bucket_scheme().as_str())
            .expect("Failed while creating bucket scheme.");

        let response_result = elastic
            .index(IndexParts::IndexId(bucket_name, id_str))
            .body(json!({
                bucket_name: bucket_schema,
            }))
            .send()
            .await;

        match response_result {
            Ok(_) => SuccessfulResponse::ok_response("Ok"),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                WebError::CreateBucket(err.to_string()).error_response()
            }
        }
    }

    async fn get_document(&self, bucket_id: &str, doc_id: &str) -> JsonResponse<Document> {
        let elastic = self.get_cxt().read().await;
        let s_path = format!("/{}/_doc/{}", bucket_id, doc_id);
        let response_result = elastic
            .send(
                Method::Get,
                s_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            log::error!("Failed while getting document {}: {}", doc_id, err);
            return Err(WebError::GetDocument(err.to_string()));
        }

        let response = response_result.unwrap();
        let common_object = response.json::<Value>().await;
        if common_object.is_err() {
            let err = common_object.err().unwrap();
            log::error!("Failed while getting documents {}: {}", doc_id, err);
            return Err(WebError::from(err));
        }

        let document_json = &common_object?[&"_source"];
        match Document::deserialize(document_json) {
            Ok(mut document) => {
                document.exclude_tokens();
                Ok(web::Json(document))
            }
            Err(err) => {
                log::error!("Failed while parsing document from response: {}", err);
                Err(WebError::GetDocument(err.to_string()))
            }
        }
    }

    async fn create_document(&self, doc_form: &Document) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let bucket_id = &doc_form.folder_id;
        let doc_id = &doc_form.content_md5;
        let to_value_result = serde_json::to_value(doc_form);
        if to_value_result.is_err() {
            let err = to_value_result.err().unwrap();
            log::error!("Failed while creating document: {}", err);
            let web_err = WebError::DocumentSerializing(err.to_string());
            return web_err.error_response();
        }

        if helper::check_duplication(&elastic, bucket_id.as_str(), doc_id.as_str()).await {
            let msg = format!("Passed document: {} already exists", doc_id);
            return WebError::CreateDocument(msg).error_response();
        }

        let status = helper::send_document(&elastic, doc_form, bucket_id.as_str()).await;
        match status.is_success() {
            true => HttpResponse::new(StatusCode::OK),
            false => {
                let msg = format!("Failed while parsing elastic response: {}", doc_id);
                log::error!("Failed while sending doc to elastic: {}", msg);
                WebError::CreateDocument(msg).error_response()
            }
        }
    }
    
    async fn create_document_preview(&self, folder_id: &str, doc_form: &DocumentPreview) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let doc_id = &doc_form.id;
        let to_value_result = serde_json::to_value(doc_form);
        if to_value_result.is_err() {
            let err = to_value_result.err().unwrap();
            log::error!("Failed while creating document: {}", err);
            let web_err = WebError::DocumentSerializing(err.to_string());
            return web_err.error_response();
        }

        let status = helper::send_document_preview(&elastic, doc_form, folder_id).await;
        match status.is_success() {
            true => HttpResponse::new(StatusCode::OK),
            false => {
                let msg = format!("Failed while parsing elastic response: {}", doc_id);
                log::error!("Failed while sending doc to elastic: {}", msg);
                WebError::CreateDocument(msg).error_response()
            }
        }
    }

    async fn update_document(&self, doc_form: &Document) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let bucket_name = &doc_form.folder_id;
        let document_id = &doc_form.document_md5;

        let ser_doc_result = serde_json::to_value(doc_form);
        if ser_doc_result.is_err() {
            let err = ser_doc_result.err().unwrap();
            let doc_name = doc_form.document_name.as_str();
            log::error!("Failed while updating document {}: {}", doc_name, err);
            let web_err = WebError::UpdateDocument(err.to_string());
            return web_err.error_response();
        }

        let document_json = ser_doc_result.unwrap();
        let s_path = format!("/{}/_doc/{}", bucket_name, document_id);
        let response_result = elastic
            .send(
                Method::Put,
                s_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(document_json.to_string().as_bytes()),
                None,
            )
            .await;

        match response_result {
            Ok(_) => SuccessfulResponse::ok_response("Ok"),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                WebError::UpdateDocument(err.to_string()).error_response()
            }
        }
    }

    async fn delete_document(&self, bucket_id: &str, doc_id: &str) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let s_path = format!("/{}/_doc/{}", bucket_id, doc_id);
        let response_result = elastic
            .send(
                Method::Delete,
                s_path.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        match response_result {
            Ok(_) => SuccessfulResponse::ok_response("Ok"),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                WebError::DeleteDocument(err.to_string()).error_response()
            }
        }
    }

    async fn move_documents(&self, folder_id: &str, document_ids: &[String]) -> HttpResponse {
        let opts = self.get_options();
        watcher::move_docs_to_folder(opts.as_ref(), folder_id, document_ids).await
        // TODO: Update documents after moving
        // if result.status().is_success() {
        //     
        // }
    }

    async fn load_file_to_bucket(&self, bucket_id: &str, file_path: &str) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let file_path_ = std::path::Path::new(file_path);
        if !file_path_.exists() {
            let err = WebError::LoadFileFailed(file_path.to_string());
            log::error!("Failed to load file `{}` to bucket: {}", file_path, err);
            return err.error_response();
        }

        let documents = loader::load_passed_file_by_path(bucket_id, file_path_)
            .into_iter()
            .map(Document::from)
            .collect::<Vec<Document>>();

        let mut docs_to_remove: Vec<usize> = Vec::default();
        for (doc_index, doc_item) in documents.iter().enumerate() {
            let bucket_id = doc_item.folder_id.as_str();
            let content_id = doc_item.content_md5.as_str();
            if helper::check_duplication(&elastic, bucket_id, content_id).await {
                docs_to_remove.push(doc_index);
            }
        }

        let futures_list = documents
            .iter()
            .enumerate()
            .filter(|(index, _)| !docs_to_remove.contains(index))
            .map(|(_, doc_form)| helper::send_document(&elastic, doc_form, bucket_id))
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await;

        let failed = futures_list
            .into_iter()
            .filter(|response| !response.is_success())
            .map(|response| response.get_path())
            .collect::<Vec<_>>();

        if !failed.is_empty() {
            let msg_str = format!("Failed while saving: {}", failed.join("\n"));
            log::error!("Common error - {}", msg_str.as_str());
            return WebError::CreateDocument(msg_str).error_response();
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

    async fn launch_watcher_analysis(&self, document_ids: &[String]) -> JsonResponse<Vec<DocumentPreview>> {
        let cxt_opts = self.get_options().as_ref();
        let docs = watcher::launch_docs_analysis(cxt_opts, document_ids).await;
        if docs.is_err() {
            let err = docs.err().unwrap();
            return Err(err);
        }
        
        let analysed_docs = docs.unwrap();
        for dp in analysed_docs.iter() {
            let _ = self.create_document_preview("history", dp).await;
        }
        
        Ok(web::Json(analysed_docs))
    }

    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .send(
                Method::Post,
                "/_search/scroll=1m",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        match response_result {
            Ok(_response) => {
                let def_vals: Vec<String> = Vec::default();
                Ok(web::Json(def_vals))
            }
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(WebError::SearchFailed(err.to_string()))
            }
        }
    }
    async fn delete_pagination_ids(&self, ids: &AllScrolls) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .clear_scroll(ClearScrollParts::ScrollId(&ids.as_slice()))
            .send()
            .await;

        match response_result {
            Ok(_) => SuccessfulResponse::ok_response("Ok"),
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                WebError::SearchFailed(err.to_string()).error_response()
            }
        }
    }

    async fn next_pagination_result(
        &self,
        curr_scroll: &NextScroll,
    ) -> PaginateJsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .scroll(ScrollParts::ScrollId(curr_scroll.get_scroll_id()))
            .pretty(true)
            .send()
            .await;

        match response_result {
            Ok(response) => {
                let documents = helper::parse_search_result(response).await;
                Ok(web::Json(documents))
            }
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(WebError::SearchFailed(err.to_string()))
            }
        }
    }

    async fn search(&self, s_params: &SearchParams) -> PaginateJsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_query(s_params);
        let buckets = s_params
            .buckets
            .to_owned()
            .unwrap_or(DEFAULT_FOLDER_NAME.to_string());

        let indexes = buckets.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, indexes.as_slice(), &body_value, s_params).await {
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

        let buckets = s_params.buckets.to_owned().unwrap_or("*".to_string());
        let indexes = buckets.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, indexes.as_slice(), &body_value, s_params).await {
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

        let buckets = s_params.buckets.to_owned().unwrap_or("*".to_string());
        let indexes = buckets.split(',').collect::<Vec<&str>>();

        match helper::search_documents(&elastic, indexes.as_slice(), &body_value, s_params).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching similar documents: {}", err);
                Err(err)
            }
        }
    }

    #[cfg(feature = "enable-chunked")]
    async fn search_chunked(&self, s_params: &SearchParams) -> PaginateJsonResponse<GroupedDocs> {
        match self.search(s_params).await {
            Ok(docs) => {
                let documents = docs.0.get_founded();
                let grouped = self.group_document_chunks(documents);
                Ok(web::Json(wrappers::scroll::PaginatedResult::new(grouped)))
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
                Ok(web::Json(wrappers::scroll::PaginatedResult::new(grouped)))
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
                Ok(web::Json(wrappers::scroll::PaginatedResult::new(grouped)))
            }
            Err(err) => {
                log::error!("Failed while searching similar documents: {}", err);
                Err(err)
            }
        }
    }
}
