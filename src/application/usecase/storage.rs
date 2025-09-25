use anyhow::anyhow;
use character_text_splitter::CharacterTextSplitter;
use std::sync::Arc;

use crate::application::services::storage::error::StorageResult;
use crate::application::services::storage::{DocumentManager, IndexManager, StorageError};
use crate::application::structures::params::CreateIndexParams;
use crate::application::structures::{Document, Index, StoredDocument};

#[cfg(feature = "enable-unique-doc-id")]
use crate::infrastructure::osearch::OpenSearchStorage;

#[derive(Clone)]
pub struct StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    max_content_size: usize,
    storage: Arc<Storage>,
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    pub fn new(storage: Arc<Storage>) -> Self {
        StorageUseCase {
            storage,
            max_content_size: 90000,
        }
    }
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    #[tracing::instrument(skip(self), level = "info")]
    pub async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<String> {
        self.storage.create_index(index).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn delete_index(&self, id: &str) -> StorageResult<()> {
        self.storage.delete_index(id).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn get_all_indexes(&self) -> StorageResult<Vec<Index>> {
        self.storage.get_all_indexes().await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn get_index(&self, id: &str) -> StorageResult<Index> {
        self.storage.get_index(id).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn store_document(
        &self,
        index: &str,
        doc: &Document,
        _force: bool,
    ) -> StorageResult<StoredDocument> {
        let _ = self.storage.get_index(index).await?;

        let Some(content) = doc.content() else {
            let err = anyhow!("empty document content: {}", doc.file_path());
            return Err(StorageError::ValidationError(err));
        };

        let document_parts = match content.len() > self.max_content_size {
            false => vec![doc.clone()],
            true => {
                let doc_parts = self.split_document(doc)?;
                doc_parts
            }
        };

        match self
            .storage
            .store_document_parts(index, &document_parts)
            .await
        {
            Ok(stored_docs) => {
                let root_doc = stored_docs.first().unwrap();
                Ok(root_doc.clone())
            }
            #[cfg(feature = "enable-unique-doc-id")]
            Err(_err) if _force => {
                let doc_id = OpenSearchStorage::gen_unique_document_id(index, doc);
                tracing::warn!(index = index, id = doc_id, "document already exists");
                self.storage.update_document(index, &doc_id, doc).await?;
                Ok(StoredDocument::new(doc_id, doc.file_path().clone()))
            }
            Err(err) => Err(err),
        }
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn store_documents(
        &self,
        index: &str,
        docs: &[Document],
    ) -> StorageResult<Vec<StoredDocument>> {
        let _ = self.storage.get_index(index).await?;

        let mut stored_docs = Vec::with_capacity(docs.len());
        for doc in docs {
            let stored_doc = self.store_document(index, doc, true).await?;
            stored_docs.push(stored_doc);
        }

        Ok(stored_docs)
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()> {
        self.storage.delete_document(index, id).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn get_document(&self, index: &str, id: &str) -> StorageResult<Document> {
        self.storage.get_document(index, id).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn update_document(
        &self,
        index: &str,
        id: &str,
        doc: &Document,
    ) -> StorageResult<()> {
        let _ = self.storage.get_index(index).await?;
        self.storage.update_document(index, id, doc).await
    }

    fn split_document(&self, doc: &Document) -> StorageResult<Vec<Document>> {
        let Some(content) = doc.content() else {
            let err = anyhow!("empty document content: {}", doc.file_path());
            return Err(StorageError::ValidationError(err));
        };

        let doc_parts = CharacterTextSplitter::new()
            .with_chunk_size(self.max_content_size)
            .split_text(content)
            .into_iter()
            .map(|it| {
                let mut doc_part = doc.clone();
                doc_part.set_content(Some(it));
                doc_part
            })
            .collect::<Vec<Document>>();

        Ok(doc_parts)
    }
}
