use std::sync::Arc;
use tracing::{Instrument, info_span};

use crate::domain::storage::StorageResult;
use crate::domain::storage::models::StoredDocumentPartsInfo;
use crate::domain::storage::models::{AllDocumentParts, LargeDocument, LargeDocumentId};
use crate::domain::storage::models::{CreateIndexParams, IndexId};
use crate::domain::storage::{IDocumentPartStorage, IIndexStorage};

#[derive(Clone)]
pub struct StorageUseCase<Storage>
where
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone,
{
    storage: Arc<Storage>,
    max_content_size: usize,
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone,
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
    Storage: IIndexStorage + IDocumentPartStorage + Send + Sync + Clone,
{
    pub async fn create_index(&self, params: &CreateIndexParams) -> StorageResult<IndexId> {
        let created_index_id = self
            .storage
            .create_index(params)
            .instrument(info_span!("create-index"))
            .await?;

        Ok(created_index_id)
    }

    pub async fn check_index_exists(&self, index_id: &IndexId) -> StorageResult<IndexId> {
        self.storage
            .get_index(index_id)
            .instrument(info_span!("check-index-exists"))
            .await
    }

    pub async fn delete_index(&self, index_id: &IndexId) -> StorageResult<()> {
        self.storage
            .delete_index(index_id)
            .instrument(info_span!("delete-index"))
            .await
    }

    pub async fn get_all_indexes(&self) -> StorageResult<Vec<IndexId>> {
        let all_indexes = self
            .storage
            .get_all_indexes()
            .instrument(info_span!("get-all-indexes"))
            .await?;

        Ok(all_indexes)
    }

    pub async fn get_index(&self, index_id: &IndexId) -> StorageResult<IndexId> {
        let index = self
            .storage
            .get_index(index_id)
            .instrument(info_span!("get-index"))
            .await?;

        Ok(index)
    }

    pub async fn get_all_document_parts(
        &self,
        index_id: &IndexId,
        large_doc_id: &LargeDocumentId,
    ) -> StorageResult<AllDocumentParts> {
        let all_document_parts = self
            .storage
            .get_document_parts(index_id, large_doc_id)
            .instrument(info_span!("get-all-document-parts"))
            .await?;

        Ok(all_document_parts)
    }

    pub async fn store_document(
        &self,
        index: &IndexId,
        large_doc: LargeDocument,
        _force: bool,
    ) -> StorageResult<StoredDocumentPartsInfo> {
        let _ = self.check_index_exists(index).await?;

        let part_size = self.max_content_size;
        let document_parts = large_doc.divide_large_document_on_parts(part_size)?;

        match self
            .storage
            .store_document_parts(index, document_parts)
            .instrument(info_span!("store-document"))
            .await
        {
            Err(err) => Err(err),
            Ok(info) => Ok(info),
        }
    }

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

    pub async fn delete_document(
        &self,
        index_id: &IndexId,
        large_doc_id: &LargeDocumentId,
    ) -> StorageResult<()> {
        self.storage
            .delete_document_parts(index_id, large_doc_id)
            .instrument(info_span!("delete-document-parts"))
            .await
    }
}

#[cfg(feature = "enable-unique-doc-id")]
pub fn gen_unique_document_id(index: &str, large_doc_id: &str, doc_part_id: usize) -> String {
    let common_file_path = format!("{index}/{}/{}", large_doc_id, doc_part_id);
    let digest = md5::compute(&common_file_path);
    format!("{digest:x}")
}
