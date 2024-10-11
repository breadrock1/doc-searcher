use crate::storage::models::InfoFolder;
use crate::storage::models::Document;
use crate::storage::models::DocumentPreview;
use crate::storage::models::DocumentVectors;
use crate::storage::DocumentsTrait;

use elasticsearch::http::request::JsonBody;
use serde_json::{json, Value};

#[async_trait::async_trait]
pub trait StoreTrait<T: DocumentsTrait + serde::Serialize> {
    async fn create_body(doc_form: &T) -> Vec<JsonBody<Value>>;
}

#[async_trait::async_trait]
impl StoreTrait<Document> for Document {
    async fn create_body(doc_form: &Document) -> Vec<JsonBody<Value>> {
        let to_value_result = serde_json::to_value(doc_form);
        let document_json = to_value_result.unwrap();
        let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

        body.push(json!({"index": { "_id": doc_form.get_doc_id() }}).into());
        body.push(document_json.into());

        body
    }
}

#[async_trait::async_trait]
impl StoreTrait<DocumentPreview> for DocumentPreview {
    async fn create_body(doc_form: &DocumentPreview) -> Vec<JsonBody<Value>> {
        let to_value_result = serde_json::to_value(doc_form);
        let document_json = to_value_result.unwrap();
        let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

        body.push(json!({"index": { "_id": doc_form.get_doc_id() }}).into());
        body.push(document_json.into());

        body
    }
}

#[async_trait::async_trait]
impl StoreTrait<DocumentVectors> for DocumentVectors {
    async fn create_body(doc_form: &DocumentVectors) -> Vec<JsonBody<Value>> {
        let mut base_doc_vecs = doc_form.clone();
        base_doc_vecs.exclude_embeddings();

        let all_embeddings = doc_form.get_embeddings();
        let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(all_embeddings.len() * 2);
        for vector in doc_form.get_embeddings() {
            let mut doc = base_doc_vecs.clone();
            doc.append_embeddings(vector.to_owned());

            let to_value_result = serde_json::to_value(doc);
            let doc_json = to_value_result.unwrap();
            body.push(json!({"index": { "_id": vector.get_id() }}).into());
            body.push(doc_json.into());
        }

        body
    }
}

#[async_trait::async_trait]
impl StoreTrait<InfoFolder> for InfoFolder {
    async fn create_body(info_folder: &InfoFolder) -> Vec<JsonBody<Value>> {
        let to_value_result = serde_json::to_value(info_folder);
        let info_folder_json = to_value_result.unwrap();
        let mut body: Vec<JsonBody<Value>> = Vec::with_capacity(2);

        body.push(json!({"index": { "_id": info_folder.index() }}).into());
        body.push(info_folder_json.into());

        body
    }
}
