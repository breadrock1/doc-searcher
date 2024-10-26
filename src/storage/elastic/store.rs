use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::storage::elastic::EsCxt;
use crate::storage::errors::StorageResult;
use crate::storage::models::{Document, DocumentVectors, DocumentsTrait, InfoFolder};

use elasticsearch::http::request::JsonBody;
use elasticsearch::params::Refresh;
use elasticsearch::{BulkParts, IndexParts};
use serde_json::{json, Value};

#[async_trait::async_trait]
pub trait StoreTrait<T: DocumentsTrait + serde::Serialize> {
    async fn create_body(form: &T) -> Vec<JsonBody<Value>>;
    async fn store(es_cxt: EsCxt, index: &str, form: &T) -> StorageResult<Successful>;
    async fn store_all(es_cxt: EsCxt, index: &str, form: &T) -> StorageResult<Successful>;
}

#[async_trait::async_trait]
impl StoreTrait<Document> for Document {
    async fn create_body(form: &Document) -> Vec<JsonBody<Value>> {
        let to_value_result = serde_json::to_value(form);
        let document_json = to_value_result.unwrap();
        let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

        body.push(json!({"index": { "_id": form.get_doc_id() }}).into());
        body.push(document_json.into());

        body
    }

    async fn store(es_cxt: EsCxt, index: &str, form: &Document) -> StorageResult<Successful> {
        let doc_id = form.get_doc_id();
        let elastic = es_cxt.write().await;
        let response = elastic
            .index(IndexParts::IndexId(index, doc_id))
            .refresh(Refresh::True)
            .timeout("1m")
            .body(&form)
            .send()
            .await?;

        let response = ElasticClient::extract_response_msg(response).await?;
        Ok(response)
    }

    async fn store_all(es_cxt: EsCxt, index: &str, form: &Document) -> StorageResult<Successful> {
        let body = Document::create_body(form).await;
        let elastic = es_cxt.write().await;
        let response = elastic
            .bulk(BulkParts::Index(index))
            .refresh(Refresh::True)
            .timeout("1m")
            .body(body)
            .send()
            .await?;

        let response = ElasticClient::extract_response_msg(response).await?;
        Ok(response)
    }
}

#[async_trait::async_trait]
impl StoreTrait<DocumentVectors> for DocumentVectors {
    async fn create_body(form: &DocumentVectors) -> Vec<JsonBody<Value>> {
        let mut base_doc_vec = form.clone();
        base_doc_vec.exclude_embeddings();

        let all_embeddings = form.embeddings();
        let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(all_embeddings.len() * 2);
        for vector in form.embeddings() {
            let mut doc = base_doc_vec.clone();
            doc.append_embeddings(vector.to_owned());

            let to_value_result = serde_json::to_value(doc);
            let doc_json = to_value_result.unwrap();
            body.push(json!({"index": { "_id": vector.chunk_id() }}).into());
            body.push(doc_json.into());
        }

        body
    }

    async fn store(
        es_cxt: EsCxt,
        index: &str,
        form: &DocumentVectors,
    ) -> StorageResult<Successful> {
        let doc_id = form.get_doc_id();
        let elastic = es_cxt.write().await;
        let response = elastic
            .index(IndexParts::IndexId(index, doc_id))
            .refresh(Refresh::True)
            .timeout("1m")
            .body(&form)
            .send()
            .await?;

        let response = ElasticClient::extract_response_msg(response).await?;
        Ok(response)
    }

    async fn store_all(
        es_cxt: EsCxt,
        index: &str,
        form: &DocumentVectors,
    ) -> StorageResult<Successful> {
        let body = DocumentVectors::create_body(form).await;
        let elastic = es_cxt.write().await;
        let response = elastic
            .bulk(BulkParts::Index(index))
            .refresh(Refresh::True)
            .timeout("1m")
            .body(body)
            .send()
            .await?;

        let response = ElasticClient::extract_response_msg(response).await?;
        Ok(response)
    }
}

#[async_trait::async_trait]
impl StoreTrait<InfoFolder> for InfoFolder {
    async fn create_body(form: &InfoFolder) -> Vec<JsonBody<Value>> {
        let to_value_result = serde_json::to_value(form);
        let info_folder_json = to_value_result.unwrap();
        let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

        body.push(json!({"index": { "_id": form.index() }}).into());
        body.push(info_folder_json.into());

        body
    }

    async fn store(es_cxt: EsCxt, index: &str, form: &InfoFolder) -> StorageResult<Successful> {
        let doc_id = form.get_doc_id();
        let elastic = es_cxt.write().await;
        let response = elastic
            .index(IndexParts::IndexId(index, doc_id))
            .refresh(Refresh::True)
            .timeout("1m")
            .body(&form)
            .send()
            .await?;

        let response = ElasticClient::extract_response_msg(response).await?;
        Ok(response)
    }

    async fn store_all(es_cxt: EsCxt, index: &str, form: &InfoFolder) -> StorageResult<Successful> {
        let body = InfoFolder::create_body(form).await;
        let elastic = es_cxt.write().await;
        let response = elastic
            .bulk(BulkParts::Index(index))
            .refresh(Refresh::True)
            .timeout("1m")
            .body(body)
            .send()
            .await?;

        let response = ElasticClient::extract_response_msg(response).await?;
        Ok(response)
    }
}
