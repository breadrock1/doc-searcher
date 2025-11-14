use anyhow::anyhow;
use character_text_splitter::CharacterTextSplitter;
use std::sync::Arc;

// TODO: Move to core? 
use crate::config::SettingsConfig;
use crate::core::storage::domain::{IDocumentStorage, IIndexStorage, StoredDocumentPart};
use crate::core::storage::domain::{DocumentPart, Index};
use crate::core::storage::domain::{StorageError, StorageResult};
use crate::core::storage::domain::{CreateIndexParams};
use crate::generic::usermanager::UserManager;
use crate::generic::usermanager::UserInfo;

#[cfg(feature = "enable-unique-doc-id")]
use crate::infrastructure::osearch::OpenSearchStorage;

#[derive(Clone)]
pub struct StorageUseCase<Storage>
where
    Storage: IIndexStorage + IDocumentStorage + Send + Sync,
{
    settings: Arc<SettingsConfig>,
    storage: Arc<Storage>,
    user_manager: Arc<Box<dyn UserManager + Send + Sync + 'static>>,
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IIndexStorage + IDocumentStorage + Send + Sync,
{
    pub fn new(
        settings: &SettingsConfig,
        storage: Arc<Storage>,
        user_manager: Arc<Box<dyn UserManager + Send + Sync + 'static>>,
    ) -> Self {
        let settings = Arc::new(settings.clone());
        StorageUseCase {
            storage,
            settings,
            user_manager,
        }
    }
}

impl<Storage> StorageUseCase<Storage>
where
    Storage: IIndexStorage + IDocumentStorage + Send + Sync,
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
    pub async fn get_all_indexes(&self, user_info: Option<&UserInfo>) -> StorageResult<Vec<Index>> {
        if cfg!(feature = "enable-user-manager") {
            let user_info = user_info.ok_or({
                let err = anyhow!("empty user info");
                StorageError::AuthenticationFailed(err)
            })?;

            let resources = self
                .user_manager
                .get_user_resource(user_info.user_id())
                .await
                .map_err(anyhow::Error::from)
                .map_err(StorageError::AuthenticationFailed)?;

            let indexes = resources
                .into_iter()
                .map(|it| it.into())
                .collect::<Vec<Index>>();

            return Ok(indexes);
        }

        // TODO: Will be removed into further releases
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
        doc: &DocumentPart,
        _force: bool,
    ) -> StorageResult<StoredDocumentPart> {
        let _ = self.storage.get_index(index).await?;
        let doc_parts = self.get_document_parts(doc).await?;
        match self.storage.store_document_parts(index, &doc_parts).await {
            Ok(stored_docs) => {
                let root_doc = stored_docs.first().unwrap();
                Ok(StoredDocumentPart::new(root_doc.clone(), doc.file_path().clone()))
            }
            #[cfg(feature = "enable-unique-doc-id")]
            Err(StorageError::DocumentAlreadyExists(_err)) if _force => {
                let doc_id = OpenSearchStorage::gen_unique_document_id(index, doc);
                tracing::warn!(index = index, id = doc_id, "document already exists");
                self.storage.update_document(index, &doc_id, doc).await?;
                Ok(StoredDocumentPart::new(doc_id, doc.file_path().clone()))
            }
            Err(err) => Err(err),
        }
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn store_documents(
        &self,
        index: &str,
        docs: &[DocumentPart],
    ) -> StorageResult<Vec<StoredDocumentPart>> {
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
    pub async fn get_document(&self, index: &str, id: &str) -> StorageResult<DocumentPart> {
        self.storage.get_document(index, id).await
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn update_document(
        &self,
        index: &str,
        id: &str,
        doc: &DocumentPart,
    ) -> StorageResult<()> {
        let _ = self.storage.get_index(index).await?;
        self.storage.update_document(index, id, doc).await
    }

    async fn get_document_parts(&self, doc: &DocumentPart) -> StorageResult<Vec<DocumentPart>> {
        let Some(content) = doc.content() else {
            let err = anyhow!("empty document content: {}", doc.file_path());
            return Err(StorageError::ValidationError(err));
        };

        let max_content_size = self.settings.max_content_size();
        let document_parts = match content.len() > max_content_size {
            false => vec![doc.clone()],
            true => self.split_document(doc)?,
        };

        Ok(document_parts)
    }

    fn split_document(&self, doc: &DocumentPart) -> StorageResult<Vec<DocumentPart>> {
        let content = doc.content().as_ref().ok_or_else(|| {
            let err = anyhow!("empty document content: {}", doc.file_path());
            StorageError::ValidationError(err)
        })?;

        let doc_parts = CharacterTextSplitter::new()
            .with_chunk_size(self.settings.max_content_size())
            .split_text(content)
            .into_iter()
            .enumerate()
            .map(|(part_id, it)| {
                let mut doc_part = doc.clone();
                doc_part.set_doc_part_id(part_id);
                doc_part.set_content(Some(it));
                doc_part
            })
            .collect::<Vec<DocumentPart>>();

        Ok(doc_parts)
    }
}
