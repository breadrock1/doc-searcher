use crate::errors::{Successful, WebResult};
use crate::forms::folders::folder::Folder;
use crate::forms::folders::forms::CreateFolderForm;
use crate::forms::folders::info::InfoFolder;
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
    async fn delete_folder(&self, folder_id: &str) -> WebResult<Successful> {
        let elastic = self.get_cxt().read().await;
        let response = helper::send_elrequest(&elastic, Method::Delete, None, folder_id).await?;
        helper::parse_elastic_response(response).await
    }
}
