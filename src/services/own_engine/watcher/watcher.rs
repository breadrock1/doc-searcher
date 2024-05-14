use crate::errors::{JsonResponse, WebError};
use crate::forms::preview::DocumentPreview;
use crate::services::own_engine::context::OtherContext;
use crate::services::service;

use actix_web::web;

#[async_trait::async_trait]
impl service::WatcherService for OtherContext {
    async fn launch_analysis(
        &self,
        _document_ids: &[String],
    ) -> JsonResponse<Vec<DocumentPreview>> {
        Ok(web::Json(Vec::default()))
    }

    async fn upload_files(
        &self,
        _name: &str,
        _path: &str,
    ) -> Result<Vec<DocumentPreview>, WebError> {
        Ok(Vec::default())
    }
}
