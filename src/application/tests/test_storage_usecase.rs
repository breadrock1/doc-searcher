use rstest::rstest;
use std::sync::Arc;

use crate::application::services::usermanager::UserManager;
use crate::application::structures::{DocumentPart, IndexBuilder, StoredDocumentPart};
use crate::application::tests::fixture::document::{build_large_document, DOC_FILE_PATH, DOC_ID};
use crate::application::tests::fixture::index::{DEFAULT_INDEX_ID, DEFAULT_INDEX_PATH};
use crate::application::tests::mock::{init_test_environment, TestEnvironment};
use crate::application::StorageUseCase;
use crate::config::ServiceConfig;

#[allow(dead_code)]
const DEFAULT_USER_ID: &str = "abfisgf9aadS";

#[rstest]
#[tokio::test]
#[cfg(feature = "enable-user-manager")]
async fn test_get_all_indexes(
    #[from(init_test_environment)] test_env: TestEnvironment,
) -> anyhow::Result<()> {
    use crate::application::structures::UserInfoBuilder;
    use crate::application::tests::fixture::resource::build_resource;

    let mut mock_um = test_env.um;
    mock_um
        .expect_get_user_resource()
        .times(1)
        .returning(move |_| {
            let resource = build_resource();
            Ok(vec![resource])
        });

    let config = ServiceConfig::new()?;
    let user_info = UserInfoBuilder::default()
        .user_id(DEFAULT_USER_ID.to_string())
        .build()?;

    let storage = Arc::new(test_env.storage);
    let um: Arc<Box<dyn UserManager + Send + Sync>> = Arc::new(Box::new(mock_um));
    let storage_uc = StorageUseCase::new(config.settings(), storage, um);

    let resources = storage_uc.get_all_indexes(Some(&user_info)).await?;
    assert_eq!(1, resources.len());

    Ok(())
}

#[rstest]
#[tokio::test]
#[cfg(feature = "enable-user-manager")]
async fn test_get_all_indexes_error(
    #[from(init_test_environment)] test_env: TestEnvironment,
) -> anyhow::Result<()> {
    let mut mock_um = test_env.um;
    mock_um.expect_get_user_resource().times(0);

    let config = ServiceConfig::new()?;
    let storage = Arc::new(test_env.storage);
    let um: Arc<Box<dyn UserManager + Send + Sync>> = Arc::new(Box::new(mock_um));
    let storage_uc = StorageUseCase::new(config.settings(), storage, um);

    let result = storage_uc.get_all_indexes(None).await;
    assert!(result.is_err());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_store_document_empty(
    #[from(init_test_environment)] test_env: TestEnvironment,
    #[from(build_large_document)] mut test_doc: DocumentPart,
) -> anyhow::Result<()> {
    test_doc.set_content(None);

    let mut mock_storage = test_env.storage;
    mock_storage
        .expect_get_index()
        .times(1)
        .returning(move |index| {
            let index = IndexBuilder::default()
                .id(index.to_string())
                .name(index.to_string())
                .path(DEFAULT_INDEX_PATH.to_string())
                .build()
                .unwrap();
            Ok(index)
        });

    let config = ServiceConfig::new()?;
    let storage = Arc::new(mock_storage);
    let um: Arc<Box<dyn UserManager + Send + Sync>> = Arc::new(Box::new(test_env.um));
    let storage_uc = StorageUseCase::new(config.settings(), storage, um);

    let result = storage_uc
        .store_document(DEFAULT_INDEX_ID, &test_doc, false)
        .await;
    assert!(result.is_err());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_store_document(
    #[from(init_test_environment)] test_env: TestEnvironment,
    #[from(build_large_document)] test_doc: DocumentPart,
) -> anyhow::Result<()> {
    let mut mock_storage = test_env.storage;
    mock_storage
        .expect_get_index()
        .times(1)
        .returning(move |index| {
            let index = IndexBuilder::default()
                .id(index.to_string())
                .name(index.to_string())
                .path(DEFAULT_INDEX_PATH.to_string())
                .build()
                .unwrap();
            Ok(index)
        });

    mock_storage
        .expect_store_document_parts()
        .times(1)
        .returning(move |_index, parts| {
            let first_part = &parts[0];
            let doc_id = DOC_ID.to_string();
            let doc_path = first_part.file_path().clone();
            let doc = StoredDocumentPart::new(doc_id, doc_path);
            Ok(vec![doc])
        });

    let config = ServiceConfig::new()?;
    let storage = Arc::new(mock_storage);
    let um: Arc<Box<dyn UserManager + Send + Sync>> = Arc::new(Box::new(test_env.um));
    let storage_uc = StorageUseCase::new(config.settings(), storage, um);

    let result = storage_uc
        .store_document(DEFAULT_INDEX_ID, &test_doc, false)
        .await;
    assert!(result.is_ok());

    let stored_doc = result.expect("empty stored document");
    assert_eq!(DOC_FILE_PATH, stored_doc.file_path);

    Ok(())
}
