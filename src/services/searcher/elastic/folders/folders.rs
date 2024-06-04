use crate::errors::{Successful, WebError, WebResult};
use crate::forms::folders::folder::Folder;
use crate::forms::folders::forms::{CreateFolderForm, DeleteFolderForm};
use crate::services::notifier::notifier;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::folders::helper as f_helper;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::FoldersService;

use elasticsearch::http::Method;
use serde_json::Value;

#[async_trait::async_trait]
impl FoldersService for ElasticContext {
    async fn get_all_folders(&self) -> WebResult<Vec<Folder>> {
        let elastic = self.get_cxt().read().await;
        let target_url = "/_cat/indices?format=json";
        let response = helper::send_elrequest(&elastic, Method::Get, None, target_url).await?;
        let folders = response.json::<Vec<Folder>>().await?;
        Ok(folders)
    }
    async fn get_folder(&self, folder_id: &str) -> WebResult<Folder> {
        let elastic = self.get_cxt().read().await;
        let target_url = format!("/{}/_stats", folder_id);
        let response =
            helper::send_elrequest(&elastic, Method::Get, None, target_url.as_str()).await?;

        let json_value = response.json::<Value>().await?;
        let folder = f_helper::extract_folder_stats(&json_value)?;
        Ok(folder)
    }
    async fn delete_folder(&self, folder_id: &str, folder_form: &DeleteFolderForm) -> WebResult<Successful> {
        let cxt_opts = self.get_options().as_ref();
        let result = notifier::remove_folder(cxt_opts, folder_id, folder_form).await?;
        if !result.is_success() {
            log::warn!("Failed to remove folder: {}", folder_id);
            return Err(WebError::DeleteFolder(folder_id.to_string()));
        }

        let elastic = self.get_cxt().read().await;
        let response =
            helper::send_elrequest(&elastic, Method::Delete, None, folder_id).await?;

        helper::parse_elastic_response(response).await
    }
    async fn create_folder(&self, folder_form: &CreateFolderForm) -> WebResult<Successful> {
        let cxt_opts = self.get_options().as_ref();
        let result = notifier::create_folder(cxt_opts, folder_form).await?;
        if !result.is_success() {
            let msg = format!("Failed to create folder: {}", folder_form.get_id());
            log::error!("{}", msg.as_str());
            return Err(WebError::CreateFolder(msg));
        }

        let elastic = self.get_cxt().read().await;
        let response = f_helper::create_index(&elastic, folder_form).await?;
        helper::parse_elastic_response(response).await
    }
}
