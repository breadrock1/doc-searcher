use metrics::{counter, histogram};
use std::sync::Arc;
use tracing::instrument;

use crate::domain::storage::StorageResult;
use crate::domain::storage::models::{AllDocumentParts, LargeDocument};
use crate::domain::storage::models::{CreateIndexParams, StoredDocumentPartsInfo};
use crate::domain::storage::{IDocumentPartStorage, IIndexStorage};
use crate::shared::kernel::{IndexId, LargeDocumentId};

#[derive(Clone)]
pub struct StorageUseCase<Storage>
where
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync,
{
    storage: Arc<Storage>,
    max_content_size: usize,
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync,
{
    pub fn new(storage: Arc<Storage>, max_content_size: usize) -> Self {
        StorageUseCase {
            storage,
            max_content_size,
        }
    }
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync,
{
    #[instrument(level = "info", skip(self))]
    pub async fn create_index(&self, params: &CreateIndexParams) -> StorageResult<IndexId> {
        let created_index_id = self.storage.create_index(params).await?;

        Ok(created_index_id)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn check_index_exists(&self, index_id: &IndexId) -> StorageResult<IndexId> {
        self.storage.get_index(index_id).await
    }
    #[instrument(level = "info", skip(self), err)]
    pub async fn delete_index(&self, index_id: &IndexId) -> StorageResult<()> {
        self.storage.delete_index(index_id).await
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_all_indexes(&self) -> StorageResult<Vec<IndexId>> {
        let all_indexes = self.storage.get_all_indexes().await?;

        Ok(all_indexes)
    }
    #[instrument(level = "info", skip(self))]
    pub async fn get_index(&self, index_id: &IndexId) -> StorageResult<IndexId> {
        let index = self.storage.get_index(index_id).await?;

        Ok(index)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn get_all_document_parts(
        &self,
        index_id: &IndexId,
        large_doc_id: &LargeDocumentId,
    ) -> StorageResult<AllDocumentParts> {
        let all_document_parts = self
            .storage
            .get_document_parts(index_id, large_doc_id)
            .await?;

        Ok(all_document_parts)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn store_document(
        &self,
        index: &IndexId,
        large_doc: LargeDocument,
        _force: bool,
    ) -> StorageResult<StoredDocumentPartsInfo> {
        let _ = self.check_index_exists(index).await?;

        let part_size = self.max_content_size;
        let document_parts = large_doc.divide_large_document_on_parts(part_size)?;

        let instant = tokio::time::Instant::now();
        let result = self
            .storage
            .store_document_parts(index, document_parts)
            .await;

        let is_error = result.is_err();
        counter!(
            "storing_operations_total",
            "storing_status" => is_error.to_string(),
        )
        .increment(1);

        histogram!(
            "docsearch_storing_duration_seconds",
            "storing_status" => is_error.to_string(),
        )
        .record(instant.elapsed().as_secs_f64());

        let stored_doc_info = result?;
        Ok(stored_doc_info)
    }

    #[instrument(level = "info", skip_all)]
    pub async fn store_documents(
        &self,
        index: &IndexId,
        large_docs: Vec<LargeDocument>,
    ) -> StorageResult<Vec<StoredDocumentPartsInfo>> {
        let _ = self.check_index_exists(index).await?;

        let mut stored_docs = Vec::with_capacity(large_docs.len());
        for doc in large_docs.into_iter() {
            let stored_doc = self.store_document(index, doc, true).await?;
            stored_docs.push(stored_doc);
        }

        Ok(stored_docs)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn delete_document(
        &self,
        index_id: &IndexId,
        large_doc_id: &LargeDocumentId,
    ) -> StorageResult<()> {
        self.storage
            .delete_document_parts(index_id, large_doc_id)
            .await
    }
}

#[cfg(feature = "enable-unique-doc-id")]
pub fn gen_unique_document_id(index: &str, large_doc_id: &str, doc_part_id: usize) -> String {
    let common_file_path = format!("{index}/{}/{}", large_doc_id, doc_part_id);
    let digest = md5::compute(&common_file_path);
    format!("{digest:x}")
}
