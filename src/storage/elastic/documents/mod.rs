pub mod helper;
pub mod retrieve;
pub mod store;
mod update;

use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::storage::documents::DocumentService;
use crate::storage::errors::StorageResult;
use crate::storage::forms::RetrieveParams;
use crate::storage::models::{Document, FolderType};

use elasticsearch::params::Refresh;
use elasticsearch::{DeleteParts, GetParts};
use serde_json::Value;

#[async_trait::async_trait]
impl DocumentService for ElasticClient {
    async fn get_documents(
        &self,
        folder_id: &str,
        folder_type: &FolderType,
        params: &RetrieveParams,
    ) -> StorageResult<Vec<Value>> {
        let es = self.es_client();
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
        let es = self.es_client();
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
        let es = self.es_client();
        let result = folder_type.create_document(es, folder_id, doc_form).await?;
        Ok(result)
    }

    async fn update_document(
        &self,
        folder_id: &str,
        value: &Value,
        folder_type: &FolderType,
    ) -> StorageResult<Successful> {
        let es = self.es_client();
        let result = folder_type.update_document(es, folder_id, value).await?;
        Ok(result)
    }

    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> StorageResult<Successful> {
        let es = self.es_client();
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
