use crate::errors::{SuccessfulResponse, WebError};
use crate::service::elastic::context::ElasticContext;
use crate::service::elastic::helper;
use crate::service::{GroupedDocs, JsonResponse, ServiceClient};

use cacher::values::VecCacherDocuments;
use cacher::AnyCacherService;
use hasher::{gen_hash, HashType};
use wrappers::bucket::{Bucket, BucketForm};
use wrappers::cluster::Cluster;
use wrappers::document::Document;
use wrappers::search_params::SearchParams;

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use elasticsearch::IndexParts;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::{json, Value};

#[async_trait::async_trait]
impl ServiceClient for ElasticContext {
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
            println!("Failed while getting clusters: {}", err);
            return Err(WebError::GetCluster(err.to_string()));
        }

        let response = response_result.unwrap();
        let resp_json = response.json::<Value>().await?;
        match serde_json::from_value::<Vec<Cluster>>(resp_json) {
            Ok(clusters) => Ok(web::Json(clusters)),
            Err(err) => {
                println!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }

    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster> {
        let elastic = self.get_cxt().read().await;
        let cluster_name = format!("/_nodes/{}", cluster_id);
        let response_result = elastic
            .send(
                Method::Get,
                cluster_name.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            println!("Failed while getting cluster {}: {}", cluster_id, err);
            return Err(WebError::DeletingCluster(err.to_string()));
        }

        let response = response_result.unwrap();
        let resp_json = response.json::<Value>().await?;
        match serde_json::from_value::<Cluster>(resp_json) {
            Ok(cluster) => Ok(web::Json(cluster)),
            Err(err) => {
                println!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }

    async fn create_cluster(&self, _cluster_id: &str) -> HttpResponse {
        let msg = "This functionality does not implemented yet!";
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
            println!("Failed while building jsob body: {}", msg);
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
                println!("Failed while parsing elastic response: {}", err);
                WebError::DeletingCluster(err.to_string()).error_response()
            }
        }
    }

    async fn get_all_buckets(&self) -> JsonResponse<Vec<Bucket>> {
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
            println!("Failed while getting buckets: {}", err);
            return Err(WebError::from(err));
        }

        let response = response_result.unwrap();
        let resp_json = response.json::<Value>().await?;
        match serde_json::from_value::<Vec<Bucket>>(resp_json) {
            Ok(buckets) => Ok(web::Json(buckets)),
            Err(err) => {
                println!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }

    async fn get_bucket(&self, bucket_id: &str) -> JsonResponse<Bucket> {
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
            println!("Failed while getting bucket {}: {}", bucket_id, err);
            return Err(WebError::from(err));
        }

        let response = response_result.unwrap();
        let json_value = response.json::<Value>().await;
        if json_value.is_err() {
            let err = json_value.err().unwrap();
            println!("Failed while getting bucket {}: {}", bucket_id, err);
            return Err(WebError::from(err));
        }

        match helper::extract_bucket_stats(&json_value.unwrap()) {
            Ok(bucket) => Ok(web::Json(bucket)),
            Err(err) => {
                println!("Failed while extracting buckets stats: {}", err);
                Err(err)
            }
        }
    }

    async fn delete_bucket(&self, bucket_id: &str) -> HttpResponse {
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
                println!("Failed while parsing elastic response: {}", err);
                WebError::DeleteBucket(err.to_string()).error_response()
            }
        }
    }

    async fn create_bucket(&self, bucket_form: &BucketForm) -> HttpResponse {
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
                println!("Failed while parsing elastic response: {}", err);
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
            println!("Failed while getting document {}: {}", doc_id, err);
            return Err(WebError::GetDocument(err.to_string()));
        }

        let response = response_result.unwrap();
        let common_object = response.json::<Value>().await;
        if common_object.is_err() {
            let err = common_object.err().unwrap();
            println!("Failed while getting documents {}: {}", doc_id, err);
            return Err(WebError::from(err));
        }

        let document_json = &common_object?[&"_source"];
        match Document::deserialize(document_json) {
            Ok(document) => Ok(web::Json(document)),
            Err(err) => {
                println!("Failed while parsing document from response: {}", err);
                Err(WebError::GetDocument(err.to_string()))
            }
        }
    }

    async fn create_document(&self, doc_form: &Document) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let bucket_id = &doc_form.bucket_uuid;
        let doc_id = &doc_form.content_md5;
        let to_value_result = serde_json::to_value(doc_form);
        if to_value_result.is_err() {
            let err = to_value_result.err().unwrap();
            println!("Failed while creating document: {}", err);
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
                println!("Failed while creating doc: {}", msg);
                WebError::CreateDocument(msg).error_response()
            }
        }
    }

    async fn update_document(&self, doc_form: &Document) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let bucket_name = &doc_form.bucket_uuid;
        let document_id = &doc_form.document_md5;

        let ser_doc_result = serde_json::to_value(doc_form);
        if ser_doc_result.is_err() {
            let err = ser_doc_result.err().unwrap();
            let doc_name = doc_form.document_name.as_str();
            println!("Failed while updating document {}: {}", doc_name, err);
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
                println!("Failed while parsing elastic response: {}", err);
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
                println!("Failed while parsing elastic response: {}", err);
                WebError::DeleteDocument(err.to_string()).error_response()
            }
        }
    }

    async fn load_file_to_bucket(&self, bucket_id: &str, file_path: &str) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let file_path_ = std::path::Path::new(file_path);
        if !file_path_.exists() {
            let err = WebError::LoadFileFailed(file_path.to_string());
            println!("Failed to load file `{}` to bucket: {}", file_path, err);
            return err.error_response();
        }

        let documents = loader::load_passed_file_by_path(file_path_)
            .into_iter()
            .map(Document::from)
            .collect::<Vec<Document>>();

        let mut docs_to_remove: Vec<usize> = Vec::default();
        for (doc_index, doc_item) in documents.iter().enumerate() {
            let bucket_id = doc_item.bucket_uuid.as_str();
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
            println!("Common error - {}", msg_str.as_str());
            return WebError::CreateDocument(msg_str).error_response();
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

    async fn search(&self, s_params: &SearchParams) -> JsonResponse<GroupedDocs> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_query(s_params);
        let buckets = s_params.buckets.to_owned().unwrap_or("*".to_string());
        let indexes = buckets.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, indexes.as_slice(), &body_value, s_params).await {
            Ok(documents) => {
                let documents = self.insert_cache(s_params, documents.0).await;
                let grouped_docs = helper::group_document_chunks(documents);
                Ok(web::Json(grouped_docs))
            }
            Err(err) => {
                println!("Failed while searching documents: {}", err);
                Err(err)
            }
        }
    }

    async fn search_tokens(&self, s_params: &SearchParams) -> JsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_query(s_params);
        let buckets = s_params.buckets.to_owned().unwrap_or("*".to_string());
        let indexes = buckets.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, indexes.as_slice(), &body_value, s_params).await {
            Ok(documents) => {
                let vec_docs = self.insert_cache(s_params, documents.0).await;
                Ok(web::Json(vec_docs))
            }
            Err(err) => {
                println!("Failed while searching documents: {}", err);
                Err(err)
            }
        }
    }

    async fn similarity(&self, s_params: &SearchParams) -> JsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_similar_query(s_params);

        let buckets = s_params.buckets.to_owned().unwrap_or("*".to_string());
        let indexes = buckets.split(',').collect::<Vec<&str>>();

        helper::search_documents(&elastic, indexes.as_slice(), &body_value, s_params).await
    }

    async fn load_cache(&self, s_params: &SearchParams) -> Option<GroupedDocs> {
        let cacher = self.get_cacher().read().await;
        let documents_opt = cacher.get_documents(s_params).await;
        let documents = documents_opt?.get_documents().to_owned();
        let grouped_docs = helper::group_document_chunks(documents);
        Some(grouped_docs)
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
