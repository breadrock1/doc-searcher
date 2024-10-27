use crate::errors::Successful;
use crate::storage::errors::StorageResult;
use crate::storage::forms::RetrieveParams;
use crate::storage::models::{Document, FolderType};

use serde_json::Value;

#[async_trait::async_trait]
pub trait DocumentService {
    async fn get_documents(
        &self,
        folder_id: &str,
        folder_type: &FolderType,
        params: &RetrieveParams,
    ) -> StorageResult<Vec<Value>>;

    async fn get_document(
        &self,
        folder_id: &str,
        doc_id: &str,
        folder_type: &FolderType,
    ) -> StorageResult<Value>;

    async fn create_document(
        &self,
        folder_id: &str,
        doc: &Document,
        folder_type: &FolderType,
    ) -> StorageResult<Successful>;

    async fn update_document(
        &self,
        folder_id: &str,
        doc: &Value,
        folder_type: &FolderType,
    ) -> StorageResult<Successful>;

    async fn delete_document(&self, folder_id: &str, doc_id: &str) -> StorageResult<Successful>;
}
