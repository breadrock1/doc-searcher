pub mod from;
mod helper;
pub mod schema;

use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::storage::elastic::documents::store::StoreTrait;
use crate::storage::elastic::folders::from::FromElasticResponse;
use crate::storage::errors::StorageResult;
use crate::storage::folders::FolderService;
use crate::storage::forms::CreateFolderForm;
use crate::storage::models::INFO_FOLDER_ID;
use crate::storage::models::{Folder, InfoFolder};
use elasticsearch::http::Method;
use serde_json::Value;

const CAT_INDICES_URL: &str = "/_cat/indices?format=json";

#[async_trait::async_trait]
impl FolderService for ElasticClient {
    async fn get_folders(&self, show_all: bool) -> StorageResult<Vec<Folder>> {
        let response = self
            .send_native_request(Method::Get, None, CAT_INDICES_URL)
            .await?;

        let folders = response.json::<Vec<Folder>>().await?;
        helper::filter_folders(self.es_client(), &folders, show_all).await
    }

    async fn get_folder(&self, folder_id: &str) -> StorageResult<Folder> {
        let target_url = format!("/{folder_id}/_stats");
        let response = self
            .send_native_request(Method::Get, None, &target_url)
            .await?
            .error_for_status_code()?;

        let value = response.json::<Value>().await?;
        let mut folder = Folder::from_value(value).await?;
        helper::load_folder_info(self.es_client(), folder_id, &mut folder).await?;

        Ok(folder)
    }

    async fn create_folder(&self, folder_form: &CreateFolderForm) -> StorageResult<Successful> {
        if let Err(err) = helper::create_index(self.es_client(), folder_form).await {
            tracing::error!(err=?err, "failed to create folder");
            return Err(err);
        }

        let es = self.es_client();
        let info_folder = InfoFolder::from(folder_form);
        match InfoFolder::store_all(es, INFO_FOLDER_ID, &info_folder).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                tracing::error!(err=?err, folder=INFO_FOLDER_ID, "failed to create folder");
                let folder_id = info_folder.index();
                if let Err(err_) = helper::delete_index(self.es_client(), folder_id).await {
                    tracing::error!(err=?err_, folder=folder_id, "failed to delete folder");
                }
                Err(err)
            }
        }
    }

    async fn delete_folder(&self, folder_id: &str) -> StorageResult<Successful> {
        if let Err(err) = helper::delete_folder_info(self.es_client(), folder_id).await {
            tracing::error!(err=?err, "failed to delete folder from info-folder");
            return Err(err);
        }

        match helper::delete_index(self.es_client(), folder_id).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                tracing::error!(err=?err, folder=folder_id, "failed to delete folder");
                Err(err)
            }
        }
    }
}
