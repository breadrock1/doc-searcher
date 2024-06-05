use crate::errors::WebResult;
use crate::forms::documents::DocumentsTrait;
use crate::forms::documents::forms::DocumentType;
use crate::forms::folders::folder::HISTORY_FOLDER_ID;
use crate::services::notifier::notifier;
use crate::services::searcher::elastic::context;
use crate::services::searcher::elastic::documents::helper as d_helper;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::{UploadedResult, WatcherService};

use serde_json::Value;

#[async_trait::async_trait]
impl WatcherService for context::ElasticContext {
    async fn analyse_docs(&self, document_ids: &[String], doc_type: &DocumentType) -> WebResult<Vec<Value>> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let mut analysed_docs = notifier::launch_analysis(cxt_opts, document_ids).await?;
        for doc_preview in analysed_docs.iter_mut() {
            let _ = d_helper::store_object(&elastic, doc_preview).await;
            doc_preview.set_folder_id(HISTORY_FOLDER_ID);
            let _ = d_helper::store_object(&elastic, doc_preview).await;
        }

        Ok(helper::to_unified_docs(analysed_docs, doc_type))
    }
    async fn upload_files(&self, name: &str, path: &str) -> UploadedResult {
        let cxt_opts = self.get_options().as_ref();
        notifier::translate_multipart_form(cxt_opts, name.to_string(), path.to_string()).await
    }
}
