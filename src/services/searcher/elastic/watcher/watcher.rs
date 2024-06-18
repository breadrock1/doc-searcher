use crate::errors::WebResult;
use crate::forms::documents::DocumentsTrait;
use crate::forms::documents::forms::DocumentType;
use crate::forms::folders::folder::HISTORY_FOLDER_ID;
use crate::services::notifier::notifier;
use crate::services::searcher::elastic::context;
use crate::services::searcher::elastic::documents::helper as d_helper;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::{UploadedResult, WatcherService};

use elasticsearch::BulkParts;
use elasticsearch::http::request::JsonBody;
use elasticsearch::params::Refresh;
use serde_json::{json, Value};

#[async_trait::async_trait]
impl WatcherService for context::ElasticContext {
    async fn analyse_docs(&self, document_ids: &[String], doc_type: &DocumentType) -> WebResult<Vec<Value>> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let mut analysed_docs = notifier::launch_analysis(cxt_opts, document_ids).await?;

        let mut bulk_history: Vec<JsonBody<Value>> = Vec::new();
        for doc in analysed_docs.iter_mut() {
            let status = d_helper::store_object(&elastic, doc.get_folder_id(), doc).await?;
            println!("{}", status.message);

            let vec_folder_id = format!("{}-vector", doc.get_folder_id());
            let status = d_helper::store_object(&elastic, vec_folder_id.as_str(), doc).await?;
            println!("{}", status.message);

            let to_value_result = serde_json::to_value(&doc);
            let document_json = to_value_result.unwrap();
            bulk_history.push(json!({"index": { "_id": doc.get_doc_id() }}).into());
            bulk_history.push(document_json.clone().into());
        }

        let response = elastic
            .bulk(BulkParts::Index(HISTORY_FOLDER_ID))
            .refresh(Refresh::True)
            .timeout("1m")
            .body(bulk_history)
            .send()
            .await?;

        let status = helper::parse_elastic_response(response).await?;
        println!("{}", status.message);

        Ok(helper::to_unified_docs(analysed_docs, doc_type))
    }
    async fn upload_files(&self, name: &str, path: &str) -> UploadedResult {
        let cxt_opts = self.get_options().as_ref();
        notifier::translate_multipart_form(cxt_opts, name.to_string(), path.to_string()).await
    }
}
