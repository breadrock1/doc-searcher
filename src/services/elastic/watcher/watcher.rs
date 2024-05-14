use crate::errors::JsonResponse;
use crate::forms::document::DocumentPreview;
use crate::forms::folder::HISTORY_FOLDER_ID;
use crate::services::elastic::context;
use crate::services::notifier::notifier;
use crate::services::searcher::{DocumentsService, UploadedResult, WatcherService};

use actix_web::web;

#[async_trait::async_trait]
impl WatcherService for context::ElasticContext {
    async fn launch_analysis(&self, document_ids: &[String]) -> JsonResponse<Vec<DocumentPreview>> {
        let cxt_opts = self.get_options().as_ref();
        let analysed_docs = notifier::launch_analysis(cxt_opts, document_ids).await?;
        for doc_preview in analysed_docs.iter() {
            let folder_id = doc_preview.get_folder_id();
            let _ = self
                .create_document_preview(HISTORY_FOLDER_ID, doc_preview)
                .await;
            let _ = self.create_document_preview(folder_id, doc_preview).await;
        }

        Ok(web::Json(analysed_docs))
    }
    async fn upload_files(&self, name: &str, path: &str) -> UploadedResult {
        let cxt_opts = self.get_options().as_ref();
        notifier::translate_multipart_form(cxt_opts, name.to_string(), path.to_string()).await
    }
}
