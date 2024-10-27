use crate::errors::Successful;
use crate::storage::errors::StorageResult;
use crate::storage::forms::CreateFolderForm;
use crate::storage::models::Folder;

#[async_trait::async_trait]
pub trait FolderService {
    async fn get_folders(&self, show_all: bool) -> StorageResult<Vec<Folder>>;
    async fn get_folder(&self, folder_id: &str) -> StorageResult<Folder>;
    async fn create_folder(&self, form: &CreateFolderForm) -> StorageResult<Successful>;
    async fn delete_folder(&self, folder_id: &str) -> StorageResult<Successful>;
}
