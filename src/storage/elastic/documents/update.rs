use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::storage::elastic::documents::helper as d_helper;
use crate::storage::elastic::EsCxt;
use crate::storage::errors::StorageResult;
use crate::storage::models::{Document, DocumentPreview, DocumentVectors, DocumentsTrait};

use chrono::Utc;
use elasticsearch::{GetParts, UpdateParts};
use serde_json::json;

async fn update_object(es_cxt: EsCxt, form: &str, doc: &Document) -> StorageResult<Successful> {
    let doc_id = doc.get_doc_id();
    let elastic = es_cxt.write().await;
    let response = elastic
        .update(UpdateParts::IndexId(form, doc_id))
        .body(&json!({ "doc": doc }))
        .send()
        .await?;

    let response = ElasticClient::extract_response_msg(response).await?;
    Ok(response)
}

#[async_trait::async_trait]
pub trait UpdateTrait<T>
where
    T: DocumentsTrait + serde::Serialize,
{
    async fn update(es_cxt: EsCxt, folder_id: &str, form: &T) -> StorageResult<Successful>;
}

#[async_trait::async_trait]
impl UpdateTrait<Document> for Document {
    async fn update(es_cxt: EsCxt, folder_id: &str, form: &Document) -> StorageResult<Successful> {
        update_object(es_cxt, folder_id, form).await
    }
}

#[async_trait::async_trait]
impl UpdateTrait<DocumentVectors> for DocumentVectors {
    async fn update(
        _es_cxt: EsCxt,
        _folder_id: &str,
        _form: &DocumentVectors,
    ) -> StorageResult<Successful> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl UpdateTrait<DocumentPreview> for DocumentPreview {
    async fn update(
        es_cxt: EsCxt,
        folder_id: &str,
        form: &DocumentPreview,
    ) -> StorageResult<Successful> {
        let elastic = es_cxt.read().await;
        let response = elastic
            .get(GetParts::IndexId(folder_id, form.id()))
            .send()
            .await?;

        let old_document: Document = d_helper::extract_document(response).await?;

        let def_datetime = Utc::now();
        let created = form.created_date().unwrap_or(&def_datetime);
        let modified = old_document.document_modified();
        let modified = modified.as_ref();
        let modified = modified.unwrap_or(&def_datetime);

        let new_doc = Document::builder()
            .folder_id(old_document.folder_id().to_owned())
            .folder_path(old_document.folder_path().to_owned())
            .document_id(old_document.document_id().to_owned())
            .document_ssdeep(old_document.document_ssdeep().to_owned())
            .document_name(form.name().to_owned())
            .document_path(form.location().to_owned())
            .document_size(form.file_size())
            .document_type(old_document.document_type().to_owned())
            .document_extension(old_document.document_extension().to_owned())
            .document_permissions(old_document.document_permissions())
            .content(old_document.content().to_owned())
            .document_created(Some(created.to_owned()))
            .document_modified(Some(modified.to_owned()))
            .quality_recognition(form.quality_recognition())
            .embeddings(old_document.embeddings().to_owned())
            .highlight(None)
            .build()
            .unwrap();

        update_object(es_cxt.clone(), folder_id, &new_doc).await
    }
}
