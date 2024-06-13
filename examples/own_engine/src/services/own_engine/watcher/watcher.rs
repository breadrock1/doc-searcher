use crate::errors::WebResult;
use crate::forms::documents::preview::DocumentPreview;
use crate::services::own_engine::context::OtherContext;
use crate::services::service;

#[async_trait::async_trait]
impl service::WatcherService for OtherContext {
    async fn launch_analysis(&self, _document_ids: &[String]) -> WebResult<Vec<DocumentPreview>> {
        Ok(Vec::default())
    }

    async fn upload_files(&self, _name: &str, _path: &str) -> WebResult<Vec<DocumentPreview>> {
        Ok(Vec::default())
    }
}
