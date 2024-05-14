use crate::errors::{JsonResponse, PaginateResponse, SuccessfulResponse, WebError};
use crate::forms::folder::{Folder, FolderForm};
use crate::forms::pagination::Paginated;
use crate::forms::preview::DocumentPreview;
use crate::forms::s_params::SearchParams;
use crate::services::own_engine::context::OtherContext;
use crate::services::service;

use actix_web::web;

#[async_trait::async_trait]
impl service::FoldersService for OtherContext {
    async fn get_all_folders(&self) -> JsonResponse<Vec<Folder>> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.buckets.read().await;
        let buckets_vec = map.values().cloned().collect::<Vec<Folder>>();

        Ok(web::Json(buckets_vec))
    }

    async fn get_folder(&self, bucket_id: &str) -> JsonResponse<Folder> {
        let cxt = self.get_cxt().read().await;
        let map = cxt.buckets.read().await;
        match map.get(bucket_id) {
            Some(bucket) => Ok(web::Json(bucket.clone())),
            None => {
                let msg = "Not exists".to_string();
                log::warn!("Failed while getting bucket {}", bucket_id);
                Err(WebError::GetFolder(msg))
            }
        }
    }

    async fn get_folder_documents(
        &self,
        _bucket_id: &str,
        _opt_params: Option<SearchParams>,
    ) -> PaginateResponse<Vec<DocumentPreview>> {
        let documents_vec = Vec::default();
        Ok(web::Json(Paginated::new(documents_vec)))
    }

    async fn delete_folder(&self, bucket_id: &str) -> Result<SuccessfulResponse, WebError> {
        let cxt = self.get_cxt().write().await;
        let uuid = bucket_id.to_string();
        let mut map = cxt.buckets.write().await;
        match map.remove(&uuid) {
            Some(_) => Ok(SuccessfulResponse::success("Ok")),
            None => {
                let msg = "Does not exist".to_string();
                log::warn!("Failed while deleting bucket: {}", msg.as_str());
                Err(WebError::DeleteFolder(msg))
            }
        }
    }

    async fn create_folder(
        &self,
        bucket_form: &FolderForm,
    ) -> Result<SuccessfulResponse, WebError> {
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
            Some(bucket) => Ok(SuccessfulResponse::success(bucket.get_uuid())),
            None => {
                let msg = format!("Created {}", bucket_form.get_id());
                log::warn!("New bucket has been created: {}", msg.as_str());
                Ok(SuccessfulResponse::success(msg.as_str()))
            }
        }
    }
}

#[cfg(test)]
mod test_folders {
    use crate::forms::folder::FolderForm;
    use crate::services::own_engine::context::OtherContext;
    use crate::services::service::FoldersService;

    use actix_web::test;

    const FOLDER_ID: &str = "common_folder";

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

        let response = other_context.delete_folder(FOLDER_ID).await;
        let err_message = response.err().unwrap();
        assert_eq!(err_message.name(), "Delete folder error");

        let folder_form = FolderForm::default();
        let response = other_context.create_folder(&folder_form).await;
        assert_eq!(response.is_ok(), true);

        let response = other_context.delete_folder(FOLDER_ID).await;
        assert_eq!(response.is_ok(), true);
    }

    #[test]
    async fn test_get_folders() {
        let other_context = OtherContext::new("test".to_string());
        let folder_form = FolderForm::default();
        let response = other_context.create_folder(&folder_form).await;
        assert_eq!(response.is_ok(), true);

        let response = other_context.get_all_folders().await;
        let folders_count = response.unwrap().0.len();
        assert_eq!(folders_count, 1);
    }

    #[test]
    async fn test_get_folder_by_id() {
        let folder_form = FolderForm::default();
        let other_context = OtherContext::new("test".to_string());
        let response = other_context.create_folder(&folder_form).await;
        assert_eq!(response.is_ok(), true);

        let get_folder_result = other_context.get_folder(FOLDER_ID).await;
        let folder = get_folder_result.unwrap().0;
        assert_eq!(folder.get_uuid(), FOLDER_ID);
    }
}
