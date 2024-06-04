use crate::errors::{Successful, WebError, WebResult};
use crate::forms::documents::document::Document;
use crate::forms::documents::DocumentsTrait;
use crate::forms::documents::forms::MoveDocsForm;
use crate::forms::folders::folder::HISTORY_FOLDER_ID;
use crate::services::notifier::notifier;
use crate::services::searcher::elastic::context;
use crate::services::searcher::elastic::documents::helper;
use crate::services::searcher::service::{UploadedResult, WatcherService};

#[async_trait::async_trait]
impl WatcherService for context::ElasticContext {
    async fn analyse_docs(&self, document_ids: &[String]) -> WebResult<Vec<Document>> {
        // TODO: Make more readable for other developers
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let mut analysed_docs = notifier::launch_analysis(cxt_opts, document_ids).await?;
        for doc_preview in analysed_docs.iter_mut() {
            let _ = helper::store_document(&elastic, doc_preview).await;
            doc_preview.set_folder_id(HISTORY_FOLDER_ID);
            let _ = helper::store_document(&elastic, doc_preview).await;
        }
        Ok(analysed_docs)
    }
    async fn move_documents(&self, folder_id: &str, move_form: &MoveDocsForm) -> WebResult<Successful> {
        let cxt_opts = self.get_options();
        let move_result = notifier::move_docs_to_folder(cxt_opts.as_ref(), folder_id, move_form)
            .await
            .map_err(WebError::from)?;

        if !move_result.is_success() {
            let msg = format!("Failed while moving documents from: {}", folder_id);
            log::error!("{}", msg.as_str());
            return Err(WebError::MoveDocuments(msg));
        }

        let mut collected_errs = Vec::default();
        for doc_id in move_form.get_doc_ids() {
            let result = helper::move_document(self, doc_id, folder_id, move_form).await;
            if result.is_err() {
                let err = result.err().unwrap();
                let msg = format!("Faile to move document {}: {}", doc_id, err);
                collected_errs.push(msg);
            }
        }

        if collected_errs.len() > 0 {
            let collected_docs = collected_errs.join(", ");
            let msg = format!("Failed while move document: {}", collected_docs);
            return Err(WebError::MoveDocuments(msg));
        }

        Ok(Successful::success("Ok"))
    }
    async fn upload_files(&self, name: &str, path: &str) -> UploadedResult {
        let cxt_opts = self.get_options().as_ref();
        notifier::translate_multipart_form(cxt_opts, name.to_string(), path.to_string()).await
    }
}
