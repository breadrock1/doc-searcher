use crate::errors::{SuccessfulResponse, WebError};
use crate::searcher::elastic::context::ElasticContext;
use crate::searcher::elastic::helper::*;
use crate::searcher::service_client::{JsonResponse, ServiceClient};
use crate::wrappers::bucket::{Bucket, BucketForm};
use crate::wrappers::cluster::Cluster;
use crate::wrappers::document::Document;
use crate::wrappers::search_params::SearchParams;

use actix_files::NamedFile;
use actix_web::{web, HttpResponse, ResponseError};
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::Method;
use elasticsearch::{BulkParts, CountParts, IndexParts};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use hasher::{gen_hash, HashType};
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
            return Err(WebError::GetCluster(err.to_string()));
        }

        let response = response_result.unwrap();
        match response.json::<Vec<Cluster>>().await {
            Err(err) => Err(WebError::from(err)),
            Ok(clusters) => Ok(web::Json(clusters)),
        }
    }

    async fn get_cluster(&self, cluster_id: &str) -> JsonResponse<Cluster> {
        let elastic = self.get_cxt().read().await;
        let cluster_name = format!("/_nodes/{}", cluster_id);
        let body = b"";
        let response_result = elastic
            .send(
                Method::Get,
                cluster_name.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(body.as_ref()),
                None,
            )
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            return Err(WebError::DeletingCluster(err.to_string()));
        }

        let response = response_result.unwrap();
        match response.json::<Cluster>().await {
            Err(err) => Err(WebError::from(err)),
            Ok(cluster) => Ok(web::Json(cluster)),
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
            let msg = String::from("Json body is None");
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
            Err(err) => WebError::DeletingCluster(err.to_string()).error_response(),
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
            return Err(WebError::from(err));
        }

        let response = response_result.unwrap();
        match response.json::<Vec<Bucket>>().await {
            Err(err) => Err(WebError::from(err)),
            Ok(buckets) => Ok(web::Json(buckets)),
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
            return Err(WebError::from(err));
        }

        let response = response_result.unwrap();
        let json_value = response.json::<Value>().await.unwrap();
        match extract_bucket_stats(&json_value) {
            Ok(bucket) => Ok(web::Json(bucket)),
            Err(err) => Err(err),
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
            Err(err) => WebError::DeleteBucket(err.to_string()).error_response(),
        }
    }

    async fn create_bucket(&self, bucket_form: &BucketForm) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let bucket_name = bucket_form.get_name();
        let hasher_res = gen_hash(HashType::MD5, bucket_name.as_bytes());
        let binding = hasher_res.unwrap();
        let id_str = binding.get_hash_data();
        let bucket_schema: Value = serde_json::from_str(create_bucket_scheme().as_str()).unwrap();
        let response_result = elastic
            .index(IndexParts::IndexId(bucket_name, id_str))
            .body(json!({
                bucket_name: bucket_schema,
            }))
            .send()
            .await;

        match response_result {
            Ok(_) => SuccessfulResponse::ok_response("Ok"),
            Err(err) => WebError::CreateBucket(err.to_string()).error_response(),
        }
    }

    async fn check_duplication(&self, bucket_id: &str, document_id: &str) -> bool {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .count(CountParts::Index(&[bucket_id]))
            .body(json!({
                "query" : {
                    "term" : {
                        "document_md5_hash" : document_id
                    }
                }
            }))
            .send()
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            println!("Failed: {}", err);
            return false;
        }

        let response = response_result.unwrap();
        let serialize_result = response.json::<Value>().await;
        match serialize_result {
            Ok(value) => {
                let count = value["count"].as_i64().unwrap_or(0);
                count > 0
            }
            Err(err) => {
                println!("{}", err);
                false
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
            return Err(WebError::GetDocument(err.to_string()));
        }

        let response = response_result.unwrap();
        let common_object = response.json::<Value>().await.unwrap();
        let document_json = &common_object[&"_source"];
        match Document::deserialize(document_json) {
            Ok(document) => Ok(web::Json(document)),
            Err(err) => Err(WebError::GetDocument(err.to_string())),
        }
    }

    async fn create_document(&self, doc_form: &Document) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let bucket_name = &doc_form.bucket_uuid;
        let document_id = &doc_form.document_md5_hash;
        let to_value_result = serde_json::to_value(doc_form);
        if to_value_result.is_err() {
            let err = to_value_result.err().unwrap();
            let web_err = WebError::DocumentSerializing(err.to_string());
            return web_err.error_response();
        }

        if self
            .check_duplication(bucket_name.as_str(), document_id.as_str())
            .await
        {
            let msg = format!("Passed document: {} already exists", document_id);
            return WebError::CreateDocument(msg).error_response();
        }

        let document_json = to_value_result.unwrap();
        let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);
        body.push(
            json!({
                "index": {
                    "_id": document_id.as_str()
                }
            })
            .into(),
        );
        body.push(document_json.into());

        let response_result = elastic
            .bulk(BulkParts::Index(bucket_name.as_str()))
            .body(body)
            .send()
            .await;

        match response_result {
            Ok(_) => SuccessfulResponse::ok_response("Ok"),
            Err(err) => WebError::CreateDocument(err.to_string()).error_response(),
        }
    }

    async fn update_document(&self, doc_form: &Document) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let bucket_name = &doc_form.bucket_uuid;
        let document_id = &doc_form.document_md5_hash;

        let ser_doc_result = serde_json::to_value(doc_form);
        if ser_doc_result.is_err() {
            let err = ser_doc_result.err().unwrap();
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
            Err(err) => WebError::UpdateDocument(err.to_string()).error_response(),
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
            Err(err) => WebError::DeleteDocument(err.to_string()).error_response(),
        }
    }

    async fn load_file_to_bucket(&self, bucket_id: &str, file_path: &str) -> HttpResponse {
        let elastic = self.get_cxt().read().await;
        let file_path_ = std::path::Path::new(file_path);
        if !file_path_.exists() {
            let err = WebError::LoadFileFailed(file_path.to_string());
            return err.error_response();
        }

        let documents = loader::load_passed_file_by_path(file_path_)
            .into_iter()
            .map(Document::from)
            .collect::<Vec<Document>>();

        let futures_list = documents
            .iter()
            .map(|doc_form| send_document(&elastic, doc_form, bucket_id))
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
            let err = WebError::CreateDocument(msg_str);
            return err.error_response();
        }

        SuccessfulResponse::ok_response("Ok")
    }

    async fn download_file(&self, _bucket_id: &str, file_path: &str) -> Option<NamedFile> {
        match actix_files::NamedFile::open_async(file_path).await {
            Ok(named_file) => Some(named_file),
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    async fn search_all(&self, s_params: &SearchParams) -> JsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = build_search_query(s_params);
        search_documents(&elastic, &["*"], &body_value, s_params).await
    }

    async fn search_bucket(
        &self,
        buckets_ids: &str,
        s_params: &SearchParams,
    ) -> JsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let indexes: Vec<&str> = buckets_ids.split(',').collect();
        let body_value = build_search_query(s_params);
        search_documents(&elastic, indexes.as_slice(), &body_value, s_params).await
    }

    async fn similar_all(&self, s_params: &SearchParams) -> JsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = build_search_similar_query(s_params);
        search_documents(&elastic, &["*"], &body_value, s_params).await
    }

    async fn similar_bucket(
        &self,
        buckets_id: &str,
        s_params: &SearchParams,
    ) -> JsonResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let indexes: Vec<&str> = buckets_id.split(',').collect();
        let body_value = build_search_similar_query(s_params);
        search_documents(&elastic, indexes.as_slice(), &body_value, s_params).await
    }
}
