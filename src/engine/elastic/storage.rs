use elasticsearch::http::Method;
use elasticsearch::params::Refresh;
use elasticsearch::{DeleteParts, GetParts};
use serde_json::Value;

use crate::engine::elastic::from::FromElasticResponse;
use crate::engine::elastic::helper;
use crate::engine::elastic::ops::store::StoreTrait;
use crate::engine::elastic::ElasticClient;
use crate::engine::error::StorageResult;
use crate::engine::form::{CreateFolderForm, RetrieveParams};
use crate::engine::model::{Document, Folder, FolderType, InfoFolder, INFO_FOLDER_ID};
use crate::engine::{DocumentService, FolderService};
use crate::errors::Successful;

const CAT_INDICES_URL: &str = "/_cat/indices?format=json";

#[async_trait::async_trait]
impl FolderService for ElasticClient {
    async fn get_folders(&self, show_all: bool) -> StorageResult<Vec<Folder>> {
        let response = self
            .send_native_request(Method::Get, None, CAT_INDICES_URL)
            .await?;

        let folders = response.json::<Vec<Folder>>().await?;
        helper::filter_folders(self.client.clone(), &folders, show_all).await
    }

    async fn get_folder(&self, folder_id: &str) -> StorageResult<Folder> {
        let target_url = format!("/{folder_id}/_stats");
        let response = self
            .send_native_request(Method::Get, None, &target_url)
            .await?
            .error_for_status_code()?;

        let value = response.json::<Value>().await?;
        let mut folder = Folder::from_value(value).await?;
        helper::load_folder_info(self.client.clone(), folder_id, &mut folder).await?;

        Ok(folder)
    }

    async fn create_folder(&self, folder_form: &CreateFolderForm) -> StorageResult<Successful> {
        if let Err(err) = helper::create_index(self.client.clone(), folder_form).await {
            tracing::error!(err=?err, "failed to create folder");
            return Err(err);
        }

        let es = self.client.clone();
        let info_folder = InfoFolder::from(folder_form);
        match InfoFolder::store_all(es, INFO_FOLDER_ID, &info_folder).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                tracing::error!(err=?err, folder=INFO_FOLDER_ID, "failed to create folder");
                let folder_id = info_folder.index();
                if let Err(err_) = helper::delete_index(self.client.clone(), folder_id).await {
                    tracing::error!(err=?err_, folder=folder_id, "failed to delete folder");
                }
                Err(err)
            }
        }
    }

    async fn delete_folder(&self, folder_id: &str) -> StorageResult<Successful> {
        if let Err(err) = helper::delete_folder_info(self.client.clone(), folder_id).await {
            tracing::error!(err=?err, "failed to delete folder from info-folder");
            return Err(err);
        }

        match helper::delete_index(self.client.clone(), folder_id).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                tracing::error!(err=?err, folder=folder_id, "failed to delete folder");
                Err(err)
            }
        }
    }
}

#[async_trait::async_trait]
impl DocumentService for ElasticClient {
    async fn get_documents(
        &self,
        folder_id: &str,
        folder_type: &FolderType,
        params: &RetrieveParams,
    ) -> StorageResult<Vec<Value>> {
        let es = self.client.clone();
        let folders = folder_id.split(',').collect::<Vec<&str>>();
        let documents = folder_type.get_all_documents(es, &folders, params).await?;
        Ok(documents)
    }

    async fn get_document(
        &self,
        folder_id: &str,
        doc_id: &str,
        folder_type: &FolderType,
    ) -> StorageResult<Value> {
        let es = self.client.clone();
        let elastic = es.write().await;
        let response = elastic
            .get(GetParts::IndexId(folder_id, doc_id))
            .refresh(true)
            .pretty(true)
            .send()
            .await?
            .error_for_status_code()?;

        let value = folder_type.get_document(response).await?;
        Ok(value)
    }

    async fn create_document(
        &self,
        folder_id: &str,
        doc_form: &Document,
        folder_type: &FolderType,
    ) -> StorageResult<Successful> {
        let es = self.client.clone();
        let result = folder_type.create_document(es, folder_id, doc_form).await?;
        Ok(result)
    }

    async fn update_document(
        &self,
        folder_id: &str,
        value: &Value,
        folder_type: &FolderType,
    ) -> StorageResult<Successful> {
        let es = self.client.clone();
        let result = folder_type.update_document(es, folder_id, value).await?;
        Ok(result)
    }

    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> StorageResult<Successful> {
        let es = self.client.clone();
        let elastic = es.read().await;
        let response = elastic
            .delete(DeleteParts::IndexId(folder_id, doc_id))
            .refresh(Refresh::True)
            .timeout("1m")
            .send()
            .await?;

        let response = ElasticClient::extract_response_msg(response).await?;
        Ok(response)
    }
}
