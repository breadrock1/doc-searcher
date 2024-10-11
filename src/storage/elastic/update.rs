use crate::storage::elastic::helper as d_helper;
use crate::storage::models::Document;
use crate::storage::models::OcrMetadata;
use crate::storage::models::DocumentPreview;
use crate::storage::DocumentsTrait;
use crate::errors::{Successful, WebResult};

use chrono::Utc;
use elasticsearch::{Elasticsearch, GetParts};
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait::async_trait]
pub trait UpdateTrait<T: DocumentsTrait + serde::Serialize> {
    async fn update(
        es_cxt: Arc<RwLock<Elasticsearch>>,
        folder_id: &str,
        doc_form: &T,
    ) -> WebResult<Successful>;
}

#[async_trait::async_trait]
impl UpdateTrait<Document> for Document {
    async fn update(
        es_cxt: Arc<RwLock<Elasticsearch>>,
        folder_id: &str,
        doc_form: &Document,
    ) -> WebResult<Successful> {
        d_helper::update_document(es_cxt, folder_id, doc_form).await
    }
}

#[async_trait::async_trait]
impl UpdateTrait<DocumentPreview> for DocumentPreview {
    async fn update(
        es_cxt: Arc<RwLock<Elasticsearch>>,
        folder_id: &str,
        doc_form: &DocumentPreview,
    ) -> WebResult<Successful> {
        let es_cxt_cln = es_cxt.clone();
        let client = es_cxt_cln.read().await;
        let response = client
            .get(GetParts::IndexId(folder_id, doc_form.id()))
            .send()
            .await?;

        let old_document: Document = d_helper::extract_document(response).await?;

        let def_datetime = Utc::now();
        let created = doc_form.created_date().unwrap_or(&def_datetime);
        let modified = old_document.document_modified();
        let modified = modified.as_ref();
        let modified = modified.unwrap_or(&def_datetime);

        let def_artifacts = Vec::default();
        let artifacts = doc_form.get_artifacts().unwrap_or(&def_artifacts);

        let ocr = old_document.ocr_metadata().map(|ocr_meta| {
            OcrMetadata::builder()
                .job_id(String::default())
                .pages_count(ocr_meta.pages_count())
                .doc_type(ocr_meta.doc_type().to_owned())
                .artifacts(Some(artifacts.to_owned()))
                .build()
                .unwrap()
        });

        let location = std::path::Path::new("./indexer").join(folder_id);
        let location_str = location.to_str().unwrap_or(folder_id);

        let new_doc = Document::builder()
            .folder_id(old_document.folder_id().to_owned())
            .folder_path(old_document.folder_path().to_owned())
            .document_id(old_document.document_id().to_owned())
            .document_ssdeep(old_document.document_ssdeep().to_owned())
            .document_name(doc_form.name().to_owned())
            .document_path(location_str.to_string())
            .document_size(doc_form.file_size())
            .document_type(old_document.document_type().to_owned())
            .document_extension(old_document.document_extension().to_owned())
            .document_permissions(old_document.document_permissions())
            .content(old_document.content().to_owned())
            .document_created(Some(created.to_owned()))
            .document_modified(Some(modified.to_owned()))
            .quality_recognition(doc_form.quality_recognition())
            .highlight(None)
            .ocr_metadata(ocr)
            .embeddings(Some(old_document.get_embeddings().to_owned()))
            .build()
            .unwrap();

        d_helper::update_document(es_cxt, folder_id, &new_doc).await
    }
}
