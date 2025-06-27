use std::sync::Arc;

use crate::application::dto::{Document, Index};
use crate::application::services::storage::error::StorageResult;
use crate::application::services::storage::{DocumentManager, IndexManager};

#[derive(Clone)]
pub struct StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    client: Arc<Storage>,
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    pub fn new(client: Arc<Storage>) -> Self {
        StorageUseCase { client }
    }
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    pub async fn create_index(&self, index: Index) -> StorageResult<Index> {
        self.client.create_index(index).await
    }

    pub async fn delete_index(&self, id: &str) -> StorageResult<()> {
        self.client.delete_index(id).await
    }

    pub async fn get_all_indexes(&self) -> StorageResult<Vec<Index>> {
        self.client.get_all_indexes().await
    }

    pub async fn get_index(&self, id: &str) -> StorageResult<Index> {
        self.client.get_index(id).await
    }

    pub async fn create_document(&self, index: &str, doc: Document) -> StorageResult<Document> {
        self.client.create_document(index, doc).await
    }

    pub async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()> {
        self.client.delete_document(index, id).await
    }

    pub async fn get_document(&self, index: &str, id: &str) -> StorageResult<Document> {
        self.client.get_document(index, id).await
    }

    pub async fn update_document(&self, index: &str, doc: Document) -> StorageResult<()> {
        self.client.update_document(index, doc).await
    }
}
