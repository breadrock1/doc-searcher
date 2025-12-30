use rstest::rstest;
use std::sync::Arc;

use crate::application::tests::fixture::document::build_large_document;
use crate::application::tests::fixture::{DEFAULT_INDEX_ID, FIRST_DOC_PART_ID, LARGE_DOC_ID};
use crate::application::tests::mock::{TestEnvironment, init_test_environment};
use crate::application::usecase::storage::StorageUseCase;
use crate::domain::storage::models::{LargeDocument, StoredDocumentPartsInfoBuilder};

const MAX_CONTENT_SIZE: usize = 1024;

#[rstest]
#[tokio::test]
async fn test_store_document_empty(
    #[from(init_test_environment)] test_env: TestEnvironment,
    #[from(build_large_document)] mut test_doc: LargeDocument,
) -> anyhow::Result<()> {
    test_doc.content = String::default();

    let mut mock_storage = test_env.storage;
    mock_storage.expect_store_document_parts().never();
    mock_storage
        .expect_get_index()
        .times(1)
        .returning(move |index| Ok(index.to_string()));

    let storage = Arc::new(mock_storage);
    let storage_uc = StorageUseCase::new(storage, MAX_CONTENT_SIZE);

    let index_id = DEFAULT_INDEX_ID.to_string();
    let result = storage_uc.store_document(&index_id, test_doc, false).await;

    assert!(result.is_err());

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_store_document(
    #[from(init_test_environment)] test_env: TestEnvironment,
    #[from(build_large_document)] test_doc: LargeDocument,
) -> anyhow::Result<()> {
    let mut mock_storage = test_env.storage;
    mock_storage
        .expect_get_index()
        .times(1)
        .returning(move |index| Ok(index.to_string()));

    mock_storage
        .expect_store_document_parts()
        .times(1)
        .returning(move |_index, parts| {
            let _first_part = &parts[0];

            let large_doc_id = LARGE_DOC_ID.to_string();
            let first_doc_part_id = FIRST_DOC_PART_ID.to_string();
            let doc_parts_amount = 10;

            let stored_doc_parts_info = StoredDocumentPartsInfoBuilder::default()
                .large_doc_id(large_doc_id)
                .first_part_id(first_doc_part_id)
                .doc_parts_amount(doc_parts_amount)
                .build()
                .expect("failed to build stored document parts information");

            Ok(stored_doc_parts_info)
        });

    let storage = Arc::new(mock_storage);
    let storage_uc = StorageUseCase::new(storage, MAX_CONTENT_SIZE);

    let index_id = DEFAULT_INDEX_ID.to_string();
    let result = storage_uc.store_document(&index_id, test_doc, false).await;
    assert!(result.is_ok());

    let stored_doc = result.expect("empty stored document");
    assert_eq!(LARGE_DOC_ID, stored_doc.large_doc_id);

    Ok(())
}
