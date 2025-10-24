use doc_search::application::services::storage::{DocumentManager, DocumentSearcher, IndexManager};
use doc_search::application::structures::Document;
use doc_search::application::structures::params::{CreateIndexParamsBuilder, FullTextSearchParams, KnnIndexParams, RetrieveDocumentParams, SemanticSearchParams};
use doc_search::config::ServiceConfig;
use doc_search::infrastructure::osearch::OpenSearchStorage;
use doc_search::ServiceConnect;
use std::sync::Arc;

const TEST_FOLDER_ID: &str = "test-common-folder";
const TEST_DOCUMENTS_DATA: &[u8] = include_bytes!("resources/test-document.json");
const TEST_FULLTEXT_DATA: &[u8] = include_bytes!("resources/fulltext-params.json");
const TEST_RETRIEVE_DATA: &[u8] = include_bytes!("tests/resources/retrieve-params.json");
const TEST_SEMANTIC_DATA: &[u8] = include_bytes!("tests/resources/semantic-params.json");

#[ignore]
#[tokio::test]
async fn test_searcher_api() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let config = config.storage().opensearch();
    let client = Arc::new(OpenSearchStorage::connect(config).await?);

    let _ = client.delete_index(TEST_FOLDER_ID).await;
    let _ = create_test_index(client.clone()).await;

    let documents = serde_json::from_slice::<Vec<Document>>(TEST_DOCUMENTS_DATA)?;
    let result = client
        .store_document_parts(TEST_FOLDER_ID, &documents)
        .await;
    assert!(result.is_ok());

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let retrieve_params = serde_json::from_slice::<RetrieveDocumentParams>(TEST_RETRIEVE_DATA)?;
    let result = client.retrieve(TEST_FOLDER_ID, &retrieve_params).await;
    assert!(result.is_ok());
    let result = result?;
    assert_eq!(result.founded().len(), 3);

    let fulltext_params = serde_json::from_slice::<FullTextSearchParams>(TEST_FULLTEXT_DATA)?;
    let result = client.fulltext(&fulltext_params).await;
    assert!(result.is_ok());
    let result = result?;
    assert_eq!(result.founded().len(), 3);

    let semantic_params = serde_json::from_slice::<SemanticSearchParams>(TEST_SEMANTIC_DATA)?;
    let result = client.semantic(&semantic_params).await;
    assert!(result.is_ok());
    let result = result?;
    assert_eq!(result.founded().len(), 3);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_documents_api() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let config = config.storage().opensearch();
    let client = Arc::new(OpenSearchStorage::connect(config).await?);

    let _ = client.delete_index(TEST_FOLDER_ID).await;
    let _ = create_test_index(client.clone()).await;

    let documents = serde_json::from_slice::<Vec<Document>>(TEST_DOCUMENTS_DATA)?;
    let result = client
        .store_document_parts(TEST_FOLDER_ID, &documents)
        .await;
    assert!(result.is_ok());

    for stored_doc in result?.iter() {
        let id = &stored_doc.id;

        let result = client.get_document(TEST_FOLDER_ID, id).await;
        assert!(result.is_ok());

        client.delete_document(TEST_FOLDER_ID, id).await?;
        let result = client.get_document(TEST_FOLDER_ID, id).await;
        assert!(result.is_err());
    }

    let _ = client.delete_index(TEST_FOLDER_ID).await;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_index_api() -> anyhow::Result<()> {
    let config = ServiceConfig::new()?;
    let config = config.storage().opensearch();
    let client = Arc::new(OpenSearchStorage::connect(config).await?);

    let _ = client.delete_index(TEST_FOLDER_ID).await;
    let _ = create_test_index(client.clone()).await;
    let loaded_index = client.get_index(TEST_FOLDER_ID).await?;
    assert_eq!(TEST_FOLDER_ID, loaded_index.id());

    client.delete_index(TEST_FOLDER_ID).await?;
    let result = client.get_index(TEST_FOLDER_ID).await;
    assert!(result.is_err());

    Ok(())
}

async fn create_test_index(client: Arc<OpenSearchStorage>) -> anyhow::Result<String> {
    let create_index = CreateIndexParamsBuilder::default()
        .id(TEST_FOLDER_ID.to_owned())
        .name(TEST_FOLDER_ID.to_owned())
        .path("".to_owned())
        .knn(Some(KnnIndexParams::default()))
        .build()
        .unwrap();

    let id = client.create_index(&create_index).await?;
    Ok(id)
}

#[test]
#[cfg(feature = "enable-unique-doc-id")]
fn test_gen_unique_document_id() -> anyhow::Result<()> {
    let documents = serde_json::from_slice::<Vec<Document>>(TEST_DOCUMENTS_DATA)?;
    for doc in documents.iter() {
        let result = OpenSearchStorage::gen_unique_document_id(TEST_FOLDER_ID, doc);
        println!("doc unique id is: {result}");
    }
    Ok(())
}
