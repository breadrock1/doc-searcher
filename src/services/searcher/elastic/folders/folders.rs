use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::forms::folders::folder::Folder;
use crate::forms::folders::forms::{CreateFolderForm, DeleteFolderForm};
use crate::forms::folders::info::InfoFolder;
use crate::services::notifier::notifier;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::documents::helper as d_helper;
use crate::services::searcher::elastic::folders::helper as f_helper;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::service::FolderService;

use elasticsearch::http::Method;
use serde_json::Value;

#[async_trait::async_trait]
impl FolderService for ElasticContext {
    async fn get_all_folders(&self, show_all: bool) -> WebResult<Vec<Folder>> {
        let ctx_opts = self.get_options();
        let elastic = self.get_cxt().read().await;
        let target_url = "/_cat/indices?format=json";
        let response = helper::send_elrequest(&elastic, Method::Get, None, target_url).await?;
        let folders = response.json::<Vec<Folder>>().await?;
        f_helper::filter_folders(&elastic, ctx_opts, folders, show_all).await
    }
    async fn get_folder(&self, folder_id: &str) -> WebResult<Folder> {
        let elastic = self.get_cxt().read().await;
        let target_url = format!("/{}/_stats", folder_id);
        let response =
            helper::send_elrequest(&elastic, Method::Get, None, target_url.as_str()).await?;

        let json_value = response.json::<Value>().await?;
        let mut folder = f_helper::extract_folder_stats(&json_value)?;
        let _ = f_helper::load_info_doc(&elastic, &mut folder).await;
        Ok(folder)
    }
    async fn create_folder(&self, folder_form: &CreateFolderForm) -> WebResult<Successful> {
        let cxt_opts = self.get_options().as_ref();
        let result = notifier::create_folder(cxt_opts, folder_form).await?;
        if !result.is_success() {
            let msg = format!("Failed to create folder: {}", folder_form.get_id());
            log::error!("{}", msg.as_str());
            let entity = WebErrorEntity::new(msg);
            return Err(WebError::CreateFolder(entity));
        }

        let elastic = self.get_cxt().read().await;
        let response = f_helper::create_index(&elastic, folder_form).await?;
        let result = helper::parse_elastic_response(response).await?;
        if result.is_success() {
            let info_folder = InfoFolder::from(folder_form);
            let res = d_helper::store_object(&elastic, "info-folder", &info_folder).await?;
            log::warn!("{} - {}", res.code, res.message);
        }
        
        Ok(result)
    }
    async fn delete_folder(&self, folder_id: &str, folder_form: &DeleteFolderForm) -> WebResult<Successful> {
        let cxt_opts = self.get_options().as_ref();
        let result = notifier::remove_folder(cxt_opts, folder_id, folder_form).await?;
        if !result.is_success() {
            log::warn!("Failed to remove folder: {}", folder_id);
            let entity = WebErrorEntity::new(folder_id.to_string());
            return Err(WebError::DeleteFolder(entity));
        }

        let elastic = self.get_cxt().read().await;
        let response = helper::send_elrequest(&elastic, Method::Delete, None, folder_id).await?;
        if result.is_success() {
            match f_helper::del_from_info_folder(&elastic, folder_id).await {
                Ok(success) => {
                    log::warn!("{} - {}", success.code, success.message);
                }
                Err(err) => {
                    log::warn!("Failed to remove from info-folder: {}", err);
                }
            }
        }

        helper::parse_elastic_response(response).await
    }
}
