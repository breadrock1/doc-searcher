use crate::errors::{JsonResponse, PaginateResponse, SuccessfulResponse, WebError};
use crate::forms::document::DocumentPreview;
use crate::forms::folder::{Folder, FolderForm};
use crate::forms::s_params::SearchParams;
use crate::forms::scroll::Paginated;
use crate::services::elastic::{context, helper};
use crate::services::notifier::notifier;
use crate::services::searcher::FoldersService;

use actix_web::web;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use elasticsearch::IndexParts;
use serde_json::{json, Value};

#[async_trait::async_trait]
impl FoldersService for context::ElasticContext {
    async fn get_all_folders(&self) -> JsonResponse<Vec<Folder>> {
        let elastic = self.get_cxt().read().await;
        let response = elastic
            .send(
                Method::Get,
                "/_cat/indices?format=json",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        if !response.status_code().is_success() {
            return Err(helper::extract_exception(response).await);
        }

        match response.json::<Vec<Folder>>().await {
            Ok(folders) => Ok(web::Json(folders)),
            Err(err) => {
                log::error!("Failed while parsing elastic response: {}", err);
                Err(WebError::from(err))
            }
        }
    }
    async fn get_folder(&self, folder_id: &str) -> JsonResponse<Folder> {
        let elastic = self.get_cxt().read().await;
        let target_url = format!("/{}/_stats", folder_id);
        let response = elastic
            .send(
                Method::Get,
                target_url.as_str(),
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        if !response.status_code().is_success() {
            return Err(helper::extract_exception(response).await);
        }

        let json_value = response.json::<Value>().await?;
        match helper::extract_folder_stats(&json_value) {
            Ok(folders) => Ok(web::Json(folders)),
            Err(err) => {
                log::error!("Failed while extracting folders stats: {}", err);
                Err(err)
            }
        }
    }
    async fn get_folder_documents(
        &self,
        folder_id: &str,
        opt_params: Option<SearchParams>,
    ) -> PaginateResponse<Vec<DocumentPreview>> {
        let elastic = self.get_cxt().read().await;
        let s_params = opt_params.unwrap_or_else(SearchParams::default);

        // TODO: Implement storing data to unrecognized folder
        if folder_id.eq("unrecognized") {
            let cxt_opts = self.get_options().as_ref();
            return match notifier::get_unrecognized_docs(cxt_opts, &s_params).await {
                Err(err) => Err(err),
                Ok(documents) => Ok(web::Json(Paginated::new(documents))),
            };
        }

        let body = helper::build_match_all_query(&s_params);
        match helper::search_documents_preview(&elastic, &s_params, &body, &[folder_id]).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(err)
            }
        }
    }
    async fn delete_folder(&self, folder_id: &str) -> Result<SuccessfulResponse, WebError> {
        let cxt_opts = self.get_options().as_ref();
        let result = notifier::remove_folder(cxt_opts, folder_id).await?;
        if !result.is_success() {
            let msg = format!("Failed to remove folder: {}", folder_id);
            log::error!("{}", msg.as_str());
            return Err(WebError::DeleteFolder(msg));
        }

        let elastic = self.get_cxt().read().await;
        let response = elastic
            .send(
                Method::Delete,
                folder_id,
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn create_folder(
        &self,
        folder_form: &FolderForm,
    ) -> Result<SuccessfulResponse, WebError> {
        let cxt_opts = self.get_options().as_ref();
        let result = notifier::create_folder(cxt_opts, folder_form).await?;
        if !result.is_success() {
            let msg = format!("Failed to create folder: {}", folder_form.get_id());
            log::error!("{}", msg.as_str());
            return Err(WebError::CreateFolder(msg));
        }

        let elastic = self.get_cxt().read().await;
        let folder_id = folder_form.get_id();
        let folder_schema = helper::create_folder_schema(folder_form.get_schema());
        let response = elastic
            .index(IndexParts::Index(folder_id))
            .body(&json!({folder_id: folder_schema}))
            .send()
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
}
