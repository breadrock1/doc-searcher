use std::sync::Arc;

use crate::application::services::storage::error::StorageResult;
use crate::application::services::storage::{DocumentManager, IndexManager, StorageError};
use crate::application::services::tokenizer::{TokenizeError, TokenizeResult};
use crate::application::structures::params::CreateIndexParams;
use crate::application::structures::{
    Document, Index, InputContentBuilder, StoredDocument, TokenizedContent,
};
use crate::application::usecase::TokenizerBoxed;

#[cfg(feature = "enable-unique-doc-id")]
use crate::infrastructure::osearch::OpenSearchStorage;

#[derive(Clone)]
pub struct StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    searcher: Arc<Storage>,
    tokenizer: Arc<TokenizerBoxed>,
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    pub fn new(searcher: Arc<Storage>, tokenizer: Arc<TokenizerBoxed>) -> Self {
        StorageUseCase {
            searcher,
            tokenizer,
        }
    }
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IndexManager + DocumentManager + Send + Sync + Clone,
{
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<String> {
        self.searcher.create_index(index).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn delete_index(&self, id: &str) -> StorageResult<()> {
        self.searcher.delete_index(id).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_all_indexes(&self) -> StorageResult<Vec<Index>> {
        self.searcher.get_all_indexes().await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_index(&self, id: &str) -> StorageResult<Index> {
        self.searcher.get_index(id).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn store_document(
        &self,
        index: &str,
        doc: &Document,
        _force: bool,
    ) -> StorageResult<String> {
        let _ = self.searcher.get_index(index).await?;
        let _tokens = self
            .tokenize_content(doc)
            .await
            .map_err(anyhow::Error::from)
            .map_err(StorageError::InternalError)?;

        // TODO: set tokens to params
        match self.searcher.store_document(index, doc).await {
            Ok(doc_id) => Ok(doc_id),
            #[cfg(feature = "enable-unique-doc-id")]
            Err(_err) if _force => {
                let doc_id = OpenSearchStorage::gen_unique_document_id(index, doc);
                tracing::warn!(index = index, id = doc_id, "document already exists");
                self.searcher.update_document(index, &doc_id, doc).await?;
                Ok(doc_id)
            }
            Err(err) => Err(err),
        }
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn store_documents(
        &self,
        index: &str,
        docs: &[Document],
    ) -> StorageResult<Vec<StoredDocument>> {
        let _ = self.searcher.get_index(index).await?;
        self.searcher.store_documents(index, docs).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()> {
        self.searcher.delete_document(index, id).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_document(&self, index: &str, id: &str) -> StorageResult<Document> {
        self.searcher.get_document(index, id).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn update_document(
        &self,
        index: &str,
        id: &str,
        doc: &Document,
    ) -> StorageResult<()> {
        let _ = self.searcher.get_index(index).await?;
        let _tokens = self
            .tokenize_content(doc)
            .await
            .map_err(anyhow::Error::from)
            .map_err(StorageError::InternalError)?;

        // TODO: set tokens to params
        self.searcher.update_document(index, id, doc).await
    }

    #[tracing::instrument(skip(self), level = "debug")]
    async fn tokenize_content(&self, doc: &Document) -> TokenizeResult<TokenizedContent> {
        let Some(content) = doc.content() else {
            return Err(TokenizeError::EmptyResponse);
        };

        let input = InputContentBuilder::default()
            .content(content.clone())
            .build()
            .map_err(TokenizeError::InputFormValidation)?;

        let tokenized_content = self.tokenizer.compute(&input).await?;
        Ok(tokenized_content)
    }
}
