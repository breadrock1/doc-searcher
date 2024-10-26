mod from;
pub mod helper;
mod retrieve;
pub mod schema;
mod store;
mod update;

use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::storage::elastic::retrieve::Retrieve;
use crate::storage::elastic::store::StoreTrait;
use crate::storage::elastic::update::UpdateTrait;
use crate::storage::errors::StorageResult;
use crate::storage::forms::{CreateFolderForm, RetrieveParams};
use crate::storage::models::INFO_FOLDER_ID;
use crate::storage::models::{Document, DocumentVectors, Folder, FolderType, InfoFolder};
use crate::storage::{DocumentService, FolderService};

use elasticsearch::http::Method;
use elasticsearch::Elasticsearch;
use from::FromElasticResponse;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

type EsCxt = Arc<RwLock<Elasticsearch>>;

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
        match helper::load_from_folder_info(self.es_client(), folder_id).await {
            Err(err) => tracing::warn!("failed to load folder name: {err:#?}"),
            Ok(info_folder) => {
                let folder_uman_name = info_folder.name().to_owned();
                folder.set_name(Some(folder_uman_name));
            }
        };

        Ok(folder)
    }

    async fn create_folder(&self, folder_form: &CreateFolderForm) -> StorageResult<Successful> {
        // TODO: Added creating folder into doc-watcher (cloud) service
        if let Err(err) = helper::create_index(self.es_client(), folder_form).await {
            tracing::error!("failed to create folder: {err:#?}");
            return Err(err);
        }

        let es = self.es_client();
        let info_folder = InfoFolder::from(folder_form);
        match InfoFolder::store_all(es, INFO_FOLDER_ID, &info_folder).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                tracing::error!("failed to sync {INFO_FOLDER_ID} while creating folder: {err:#?}");
                let folder_id = info_folder.index();
                if let Err(err_) = helper::delete_index(self.es_client(), folder_id).await {
                    tracing::error!("failed to delete created folder {folder_id}: {err_:#?}");
                }
                Err(err)
            }
        }
    }

    async fn delete_folder(&self, folder_id: &str) -> StorageResult<Successful> {
        if let Err(err) = helper::delete_from_folder_info(self.es_client(), folder_id).await {
            tracing::error!("failed to delete folder from folder-info: {err:#?}");
            return Err(err);
        }

        match helper::delete_index(self.es_client(), folder_id).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                tracing::error!("failed to delete folder {folder_id}: {err:#?}");
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
        let es = self.es_client();
        let results = (params.result_size(), params.result_offset());
        let folders = folder_id.split(',').collect::<Vec<&str>>();

        match folder_type {
            FolderType::Vectors => {
                let query = DocumentVectors::build_retrieve_query(params).await;
                let response =
                    ElasticClient::search_request(es, &query, None, &folders, results).await?;
                let value = helper::extract_from_response::<DocumentVectors>(response).await?;
                Ok(value)
            }
            _ => {
                let query = Document::build_retrieve_query(params).await;
                let response =
                    ElasticClient::search_request(es, &query, None, &folders, results).await?;
                let value = helper::extract_from_response::<Document>(response).await?;
                Ok(value)
            }
        }
    }

    async fn get_document(
        &self,
        folder_id: &str,
        doc_id: &str,
        folder_type: &FolderType,
    ) -> StorageResult<Value> {
        let s_doc_path = format!("/{}/_doc/{}", folder_id, doc_id);
        let response = self
            .send_native_request(Method::Get, None, &s_doc_path)
            .await?
            .error_for_status_code()?;

        match folder_type {
            FolderType::Vectors => {
                let doc = helper::extract_document::<DocumentVectors>(response).await?;
                let value = serde_json::to_value(doc)?;
                Ok(value)
            }
            _ => {
                let doc = helper::extract_document::<Document>(response).await?;
                let value = serde_json::to_value(doc)?;
                Ok(value)
            }
        }
    }

    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> StorageResult<Successful> {
        let es = self.es_client();
        helper::delete_document(es, folder_id, doc_id).await
    }

    async fn create_document(
        &self,
        folder_id: &str,
        doc_form: &Document,
        folder_type: &FolderType,
    ) -> StorageResult<Successful> {
        let es = self.es_client();
        match folder_type {
            FolderType::Vectors => {
                let docs_vec = DocumentVectors::from(doc_form);
                DocumentVectors::store_all(es, folder_id, &docs_vec).await
            }
            _ => {
                let mut doc_cln = doc_form.to_owned();
                doc_cln.exclude_tokens();
                Document::store_all(es, folder_id, &doc_cln).await
            }
        }
    }

    async fn update_document(
        &self,
        folder_id: &str,
        value: &Value,
        folder_type: &FolderType,
    ) -> StorageResult<Successful> {
        let es = self.es_client();
        match folder_type {
            FolderType::Vectors => {
                let doc = DocumentVectors::deserialize(value)?;
                DocumentVectors::update(es, folder_id, &doc).await
            }
            _ => {
                let doc = Document::deserialize(value)?;
                Document::update(es, folder_id, &doc).await
            }
        }
    }
}
