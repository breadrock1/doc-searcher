mod common;
use common::fixture::document::*;
use common::fixture::search_params::*;
use common::setup_osearch_environment;

use doc_search_core::application::usecase::searcher::SearcherUseCase;
use doc_search_core::application::usecase::storage::StorageUseCase;
use doc_search_core::domain::searcher::models::{Pagination, SearchingParams};
use doc_search_core::domain::storage::models::{AllDocumentParts, StoredDocumentPartsInfo};
use rstest::rstest;
use serial_test::serial;

const TEST_INDEX_ID: &str = "test-folder";
const MAX_CONTENT_SIZE: usize = 10;

#[rstest]
#[serial]
#[tokio::test]
async fn test_opensearch_store_document() -> anyhow::Result<()> {
    let test_env = setup_osearch_environment(TEST_INDEX_ID).await?;
    let storage = StorageUseCase::new(test_env.osearch(), MAX_CONTENT_SIZE);

    let index_id = test_env.get_index();
    let large_document = build_large_document();
    let result: anyhow::Result<AllDocumentParts> = {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let stored_doc_info = storage
            .store_document(index_id, large_document, false)
            .await?;

        println!("document: {stored_doc_info:?} has been stored into index: {index_id}");

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let large_doc_id = &stored_doc_info.large_doc_id;
        let doc_parts = storage
            .get_all_document_parts(index_id, large_doc_id)
            .await?;
        Ok(doc_parts)
    };

    test_env.teardown().await?;

    let founded_doc_parts = result?;
    assert_eq!(founded_doc_parts.len(), 1);

    Ok(())
}

#[rstest]
#[serial]
#[tokio::test]
async fn test_opensearch_delete_documents() -> anyhow::Result<()> {
    let test_env = setup_osearch_environment(TEST_INDEX_ID).await?;
    let storage = StorageUseCase::new(test_env.osearch(), MAX_CONTENT_SIZE);

    let index_id = test_env.get_index();
    let large_document = build_large_document();
    let result: anyhow::Result<AllDocumentParts> = {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let stored_doc_info = storage
            .store_document(index_id, large_document, false)
            .await?;

        println!("document: {stored_doc_info:?} has been stored into index: {index_id}");

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let large_doc_id = &stored_doc_info.large_doc_id;
        storage.delete_document(index_id, large_doc_id).await?;

        println!("document: {large_doc_id} has been deleted");

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let doc_parts = storage
            .get_all_document_parts(index_id, large_doc_id)
            .await?;

        Ok(doc_parts)
    };

    let founded_doc_parts = result?;
    assert_eq!(founded_doc_parts.len(), 0);

    Ok(())
}

#[rstest]
#[serial]
#[tokio::test]
async fn test_opensearch_store_multiple_documents() -> anyhow::Result<()> {
    let test_env = setup_osearch_environment(TEST_INDEX_ID).await?;
    let storage = StorageUseCase::new(test_env.osearch(), MAX_CONTENT_SIZE);

    let index_id = test_env.get_index();
    let large_documents = build_real_large_documents()?;
    let result: anyhow::Result<Vec<StoredDocumentPartsInfo>> = {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let stored_docs_info = storage.store_documents(index_id, large_documents).await?;
        println!("documents: {stored_docs_info:?} has been stored into index: {index_id}");
        Ok(stored_docs_info)
    };

    let stored_docs_info = result?;
    assert_eq!(stored_docs_info.len(), 3);

    Ok(())
}

#[rstest]
#[case(build_simple_retrieve_params(), 3)]
#[case(build_simple_fulltext_params(), 3)]
#[case(build_simple_semantic_params(), 3)]
#[case(build_simple_hybrid_params(), 3)]
#[serial]
#[tokio::test]
async fn test_opensearch_search_documents(
    #[case] searching_params: SearchingParams,
    #[case] founded_documents_amount: usize,
) -> anyhow::Result<()> {
    let test_env = setup_osearch_environment(TEST_INDEX_ID).await?;
    let searcher = SearcherUseCase::new(test_env.osearch());
    let storage = StorageUseCase::new(test_env.osearch(), MAX_CONTENT_SIZE);

    let index_id = test_env.get_index();
    let large_documents = build_real_large_documents()?;
    let result: anyhow::Result<Pagination> = {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let _ = storage.store_documents(index_id, large_documents).await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let pagination = searcher.search_document_parts(&searching_params).await?;

        Ok(pagination)
    };

    let pagination = result?;
    assert_eq!(pagination.founded.len(), founded_documents_amount);

    Ok(())
}
