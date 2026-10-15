use rstest::rstest;
use std::sync::Arc;

use crate::application::tests::fixture::document::build_large_document;
use crate::application::tests::fixture::{DEFAULT_INDEX_ID, FIRST_DOC_PART_ID, LARGE_DOC_ID};
use crate::application::tests::mock::{TestEnvironment, init_test_environment};
use crate::application::usecase::storage::StorageUseCase;
use crate::domain::storage::models::{LargeDocument, StoredDocumentPartsInfoBuilder};
use crate::shared::kernel::{DocumentPartId, IndexId, LargeDocumentId, Tokens};

const MAX_CONTENT_SIZE: usize = 1024;

#[rstest]
#[tokio::test]
async fn test_store_document_empty(
    #[from(init_test_environment)] test_env: TestEnvironment,
    #[from(build_large_document)] mut test_doc: LargeDocument,
) -> anyhow::Result<()> {
    test_doc.content = String::default();

    let TestEnvironment {
        storage: mut mock_storage,
        embeddings,
    } = test_env;
    mock_storage.expect_store_document_parts().never();
    mock_storage
        .expect_get_index()
        .times(1)
        .returning(move |index| Ok(index.clone()));

    let storage_uc = StorageUseCase::new(
        Arc::new(mock_storage),
        Arc::new(embeddings),
        MAX_CONTENT_SIZE,
    );

    let index_id = IndexId(DEFAULT_INDEX_ID.to_string());
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
    let TestEnvironment {
        storage: mut mock_storage,
        embeddings: mut mock_embeddings,
    } = test_env;
    mock_embeddings
        .expect_compute()
        .returning(|_| Ok(vec![0.1, 0.2].into()));
    let embeddings = Arc::new(mock_embeddings);

    mock_storage
        .expect_get_index()
        .times(1)
        .returning(move |index| Ok(index.clone()));

    mock_storage
        .expect_store_document_parts()
        .times(1)
        .returning(move |_index, parts| {
            assert!(parts[0].embeddings.is_some()); // часть получила вектор

            let large_doc_id = LARGE_DOC_ID.to_string();
            let first_doc_part_id = FIRST_DOC_PART_ID.to_string();
            let doc_parts_amount = 10;

            let stored_doc_parts_info = StoredDocumentPartsInfoBuilder::default()
                .large_doc_id(LargeDocumentId(large_doc_id))
                .first_part_id(DocumentPartId(first_doc_part_id))
                .doc_parts_amount(doc_parts_amount)
                .build()
                .expect("failed to build stored document parts information");

            Ok(stored_doc_parts_info)
        });

    let storage_uc = StorageUseCase::new(Arc::new(mock_storage), embeddings, MAX_CONTENT_SIZE);

    let index_id = IndexId(DEFAULT_INDEX_ID.to_string());
    let result = storage_uc.store_document(&index_id, test_doc, false).await;
    assert!(result.is_ok());

    let stored_doc = result.expect("empty stored document");
    assert_eq!(LARGE_DOC_ID, stored_doc.large_doc_id.0);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_store_document_embeddings_unavailable_stores_without_vectors(
    #[from(init_test_environment)] test_env: TestEnvironment,
    #[from(build_large_document)] test_doc: LargeDocument,
) -> anyhow::Result<()> {
    let TestEnvironment {
        storage: mut mock_storage,
        embeddings: mut mock_embeddings,
    } = test_env;

    mock_embeddings
        .expect_compute()
        .returning(|_| Ok(Tokens::default()));

    let embeddings = Arc::new(mock_embeddings);

    mock_storage
        .expect_get_index()
        .times(1)
        .returning(move |index| Ok(index.clone()));
    mock_storage
        .expect_store_document_parts()
        .times(1)
        .returning(move |_index, parts| {
            assert!(parts[0].embeddings.is_none());
            Ok(StoredDocumentPartsInfoBuilder::default()
                .large_doc_id(LargeDocumentId(LARGE_DOC_ID.to_string()))
                .first_part_id(DocumentPartId(FIRST_DOC_PART_ID.to_string()))
                .doc_parts_amount(10)
                .build()
                .expect("build stored info"))
        });

    let storage_uc = StorageUseCase::new(Arc::new(mock_storage), embeddings, MAX_CONTENT_SIZE);
    let result = storage_uc
        .store_document(&IndexId(DEFAULT_INDEX_ID.to_string()), test_doc, false)
        .await;
    assert!(result.is_ok());
    Ok(())
}
