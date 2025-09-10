use std::sync::Arc;

use crate::application::services::storage::error::StorageResult;
use crate::application::services::storage::{DocumentManager, IndexManager};
use crate::application::structures::params::CreateIndexParams;
use crate::application::structures::{Document, Index, StoredDocument};

#[cfg(feature = "enable-unique-doc-id")]
use crate::infrastructure::osearch::OpenSearchStorage;

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
    pub async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<String> {
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

    pub async fn store_document(
        &self,
        index: &str,
        doc: &Document,
        _force: bool,
    ) -> StorageResult<String> {
        let _ = self.client.get_index(index).await?;
        match self.client.store_document(index, doc).await {
            Ok(doc_id) => Ok(doc_id),
            #[cfg(feature = "enable-unique-doc-id")]
            Err(_err) if _force => {
                let doc_id = OpenSearchStorage::gen_unique_document_id(index, doc);
                tracing::warn!(index = index, id = doc_id, "document already exists");
                self.client.update_document(index, &doc_id, doc).await?;
                Ok(doc_id)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn store_documents(
        &self,
        index: &str,
        docs: &[Document],
    ) -> StorageResult<Vec<StoredDocument>> {
        let _ = self.client.get_index(index).await?;
        self.client.store_documents(index, docs).await
    }

    pub async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()> {
        self.client.delete_document(index, id).await
    }

    pub async fn get_document(&self, index: &str, id: &str) -> StorageResult<Document> {
        self.client.get_document(index, id).await
    }

    pub async fn update_document(
        &self,
        index: &str,
        id: &str,
        doc: &Document,
    ) -> StorageResult<()> {
        let _ = self.client.get_index(index).await?;
        self.client.update_document(index, id, doc).await
    }
}
