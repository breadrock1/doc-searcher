use doc_search::errors::{Successful, WebError, WebResult};
use doc_search::forms::folders::folder::Folder;
use doc_search::forms::folders::forms::FolderForm;
use doc_search::services::own_engine::context::OtherContext;
use doc_search::services::service::FoldersService;

#[async_trait::async_trait]
impl FoldersService for OtherContext {
    async fn get_all_folders(&self) -> WebResult<Vec<Folder>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.buckets.read().await;
        let buckets_vec = map.values().cloned().collect::<Vec<Folder>>();
        Ok(buckets_vec)
    }
    async fn get_folder(&self, bucket_id: &str) -> WebResult<Folder> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.buckets.read().await;
        match map.get(bucket_id) {
            Some(bucket) => Ok(bucket.clone()),
            None => {
                let msg = "Not exists".to_string();
                log::warn!("Failed while getting bucket {}", bucket_id);
                Err(WebError::GetFolder(msg))
            }
        }
    }
    async fn delete_folder(&self, folder_form: &FolderForm) -> WebResult<Successful> {
        let cxt = self.get_cxt().write().await;
        let uuid = folder_form.get_id();
        let mut map = cxt.buckets.write().await;
        match map.remove(uuid) {
            Some(_) => Ok(Successful::success("Ok")),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while deleting bucket: {}", msg.as_str());
                Err(WebError::DeleteFolder(msg))
            }
        }
    }
    async fn create_folder(&self, bucket_form: &FolderForm) -> WebResult<Successful> {
        let cxt = self.get_cxt().write().await;
        let uuid = bucket_form.get_id().to_string();
        let built_bucket = Folder::builder()
            .health("health".to_string())
            .status("status".to_string())
            .index(uuid.clone())
            .uuid(uuid.clone())
            .docs_count(Some("docs_count".to_string()))
            .store_size(Some("store_size".to_string()))
            .docs_deleted(Some("docs_deleted".to_string()))
            .pri_store_size(Some("pri_store_size".to_string()))
            .pri(None)
            .rep(None)
            .build();

        let mut map = cxt.buckets.write().await;
        match map.insert(uuid, built_bucket.unwrap()) {
            Some(bucket) => Ok(Successful::success(bucket.get_uuid())),
            None => {
                let msg = format!("Created {}", bucket_form.get_id());
                log::warn!("New bucket has been created: {}", msg.as_str());
                Ok(Successful::success(msg.as_str()))
            }
        }
    }
}

#[cfg(test)]
mod test_folders {
    use crate::forms::folders::forms::FolderForm;
    use crate::services::own_engine::context::OtherContext;
    use crate::services::service::FoldersService;

    use actix_web::test;

    const FOLDER_ID: &str = "common-folder";

    #[test]
    async fn test_create_folder() {
        let folder_form = FolderForm::default();
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_folder(&folder_form).await;
        assert_eq!(response.unwrap().code, 200_u16);
    }

    #[test]
    async fn test_delete_folder() {
        let other_context = OtherContext::new("test".to_string());

        let folder_form = FolderForm::default();
        let response = other_context.delete_folder(&folder_form).await;
        let err_message = response.err().unwrap();
        assert_eq!(err_message.name(), "Delete folder error");

        let folder_form = FolderForm::default();
        let response = other_context.create_folder(&folder_form).await;
        assert_eq!(response.is_ok(), true);

        let response = other_context.delete_folder(&folder_form).await;
        assert_eq!(response.is_ok(), true);
    }

    #[test]
    async fn test_get_folders() {
        let other_context = OtherContext::new("test".to_string());
        let folder_form = FolderForm::default();
        let response = other_context.create_folder(&folder_form).await;
        assert_eq!(response.is_ok(), true);

        let response = other_context.get_all_folders().await;
        let folders_count = response.unwrap().len();
        assert_eq!(folders_count, 1);
    }

    #[test]
    async fn test_get_folder_by_id() {
        let folder_form = FolderForm::default();
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_folder(&folder_form).await;
        assert_eq!(response.is_ok(), true);

        let get_folder_result = other_context.get_folder(FOLDER_ID).await;
        let folder = get_folder_result.unwrap();
        assert_eq!(folder.get_uuid(), FOLDER_ID);
    }
}
