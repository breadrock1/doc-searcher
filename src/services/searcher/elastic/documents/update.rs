use crate::errors::{Successful, WebResult};
use crate::forms::documents::document::Document;
use crate::forms::documents::metadata::OcrMetadata;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::DocumentsTrait;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::documents::helper as d_helper;
use crate::services::searcher::service::DocumentService;

use chrono::Utc;

#[async_trait::async_trait]
pub trait UpdateTrait<T: DocumentsTrait + serde::Serialize> {
    async fn update(
        es_cxt: &ElasticContext,
        folder_id: &str,
        doc_form: &T,
    ) -> WebResult<Successful>;
}

#[async_trait::async_trait]
impl UpdateTrait<Document> for Document {
    async fn update(
        es_cxt: &ElasticContext,
        folder_id: &str,
        doc_form: &Document,
    ) -> WebResult<Successful> {
        d_helper::update_document(es_cxt, folder_id, doc_form).await
    }
}

#[async_trait::async_trait]
impl UpdateTrait<DocumentPreview> for DocumentPreview {
    async fn update(
        es_cxt: &ElasticContext,
        folder_id: &str,
        doc_form: &DocumentPreview,
    ) -> WebResult<Successful> {
        let old_doc = es_cxt
            .get_document(folder_id, doc_form.get_doc_id())
            .await?;

        let def_datetime = Utc::now();
        let created = doc_form.get_created_date().unwrap_or(&def_datetime);
        let modified = old_doc.get_doc_modified().unwrap_or(&def_datetime);

        let def_artifacts = Vec::default();
        let artifacts = doc_form.get_artifacts().unwrap_or(&def_artifacts);

        let ocr = old_doc.get_ocr_metadata().map(|ocr_meta| {
            OcrMetadata::builder()
                .job_id(String::default())
                .pages_count(ocr_meta.get_pages_count())
                .doc_type(ocr_meta.get_doc_type().to_owned())
                .artifacts(Some(artifacts.to_owned()))
                .build()
                .unwrap()
        });

        let location = std::path::Path::new("./indexer").join(folder_id);
        let location_str = location.to_str().unwrap_or(folder_id);

        let new_doc = Document::builder()
            .folder_id(old_doc.get_folder_id().to_owned())
            .folder_path(old_doc.get_folder_path().to_owned())
            .document_id(old_doc.get_doc_id().to_owned())
            .document_ssdeep(old_doc.get_doc_ssdeep().to_owned())
            .document_name(doc_form.get_name().to_string())
            .document_path(location_str.to_string())
            .document_size(doc_form.get_size())
            .document_type(old_doc.get_doc_type().to_owned())
            .document_extension(old_doc.get_doc_ext().to_owned())
            .document_permissions(old_doc.get_doc_perm().to_owned())
            .content(old_doc.get_content().to_owned())
            .document_created(Some(created.to_owned()))
            .document_modified(Some(modified.to_owned()))
            .quality_recognition(doc_form.get_quality())
            .highlight(None)
            .ocr_metadata(ocr)
            .embeddings(Some(old_doc.get_embeddings().to_owned()))
            .build()
            .unwrap();

        d_helper::update_document(es_cxt, folder_id, &new_doc).await
    }
}
