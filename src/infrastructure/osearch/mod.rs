pub mod config;
mod dto;
mod error;
mod extractor;
mod query;
mod schema;

use opensearch::auth::Credentials;
use opensearch::cat::CatIndicesParts;
use opensearch::cert::CertificateValidation;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::Url;
use opensearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use opensearch::ingest::IngestPutPipelineParts;
use opensearch::OpenSearch;
use opensearch::{
    ClearScrollParts, CreateParts, DeleteParts, GetParts, ScrollParts, SearchParts, UpdateParts,
};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::application::dto::{Document, FoundedDocument, Index};
use crate::application::dto::{
    FullTextSearchParams, PaginateParams, QueryBuilder, RetrieveDocumentParams,
    SemanticSearchParams, SemanticSearchWithTokensParams,
};
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::application::services::storage::{PaginateResult, StorageError, StorageResult};
use crate::infrastructure::osearch::config::OSearchConfig;
use crate::infrastructure::osearch::dto::SourceDocument;
use crate::ServiceConnect;

const SCROLL_LIFETIME: &str = "5m";

#[derive(Clone)]
pub struct OpenSearchStorage {
    config: OSearchConfig,
    client: Arc<OpenSearch>,
}

#[async_trait::async_trait]
impl ServiceConnect for OpenSearchStorage {
    type Config = OSearchConfig;
    type Client = OpenSearchStorage;
    type Error = opensearch::Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let es_url = Url::parse(config.address())?;
        let conn_pool = SingleNodeConnectionPool::new(es_url);

        let es_user = config.username();
        let es_passwd = config.password();
        let creds = Credentials::Basic(es_user.into(), es_passwd.into());
        let validation = CertificateValidation::None;
        let transport = TransportBuilder::new(conn_pool)
            .auth(creds)
            .cert_validation(validation)
            .build()?;

        tracing::info!(address = config.address(), "connected to elasticsearch");
        let client = OpenSearch::new(transport);
        let arc_client = Arc::new(client);
        Ok(OpenSearchStorage {
            config: config.clone(),
            client: arc_client,
        })
    }
}

#[async_trait::async_trait]
impl IndexManager for OpenSearchStorage {
    async fn create_index(&self, index: Index) -> StorageResult<Index> {
        let id = index.id();
        let model_id = self.config.model_id();
        let ingest_schema = schema::create_ingest_schema(model_id);
        let response = self
            .client
            .ingest()
            .put_pipeline(IngestPutPipelineParts::Id(schema::SEARCH_PIPELINE_NAME))
            .body(ingest_schema)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let folder_schema = schema::create_document_schema();
        let response = self
            .client
            .indices()
            .create(IndicesCreateParts::Index(id))
            .body(folder_schema)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(index)
    }

    async fn delete_index(&self, id: &str) -> StorageResult<()> {
        let response = self
            .client
            .indices()
            .delete(IndicesDeleteParts::Index(&[id]))
            .timeout("1m")
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(())
    }

    async fn get_all_indexes(&self) -> StorageResult<Vec<Index>> {
        let response = self
            .client
            .cat()
            .indices(CatIndicesParts::None)
            .format("json")
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let indexes = response
            .json::<Vec<dto::OSearchIndex>>()
            .await?
            .iter()
            .map(Index::from)
            .collect::<Vec<Index>>();

        Ok(indexes)
    }

    async fn get_index(&self, id: &str) -> StorageResult<Index> {
        let response = self
            .client
            .cat()
            .indices(CatIndicesParts::Index(&[id]))
            .format("json")
            .send()
            .await?
            .error_for_status_code()?;

        let indexes = response
            .json::<Vec<dto::OSearchIndex>>()
            .await?
            .iter()
            .map(Index::from)
            .collect::<Vec<Index>>();

        let Some(index) = indexes.first() else {
            let err = anyhow::Error::msg("there is no index with such name");
            return Err(StorageError::IndexNotFound(err));
        };

        Ok(index.to_owned())
    }
}

#[async_trait::async_trait]
impl DocumentManager for OpenSearchStorage {
    async fn create_document(&self, index: &str, doc: Document) -> StorageResult<()> {
        let response = self
            .client
            .create(CreateParts::IndexId(index, doc.id()))
            .pipeline(schema::SEARCH_PIPELINE_NAME)
            .body(&doc)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(())
    }

    async fn get_document(&self, index: &str, id: &str) -> StorageResult<Document> {
        let response = self
            .client
            .get(GetParts::IndexId(index, id))
            .pretty(true)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let document: Document = response.json::<SourceDocument>().await?.into();
        Ok(document)
    }

    async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()> {
        let response = self
            .client
            .delete(DeleteParts::IndexId(index, id))
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(())
    }

    async fn update_document(&self, index: &str, doc: Document) -> StorageResult<()> {
        let response = self
            .client
            .update(UpdateParts::IndexId(index, doc.id()))
            .body(&json!({ "doc": doc }))
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl DocumentSearcher for OpenSearchStorage {
    async fn retrieve(&self, params: &RetrieveDocumentParams) -> PaginateResult<FoundedDocument> {
        let query = params.build_query(None);
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let response = self
            .client
            .search(SearchParts::Index(&indexes))
            .pretty(true)
            .scroll(SCROLL_LIFETIME)
            .from(params.result().offset())
            .size(params.result().size())
            .body(query)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data).await?;
        Ok(paginated)
    }

    async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<FoundedDocument> {
        let query = params.build_query(None);
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let request = self
            .client
            .search(SearchParts::Index(&indexes))
            .size(params.result().size())
            .pretty(true)
            .body(query);

        let request = match params.result().offset() > 0 {
            true => request.from(params.result().offset()),
            false => request.scroll(SCROLL_LIFETIME),
        };

        let response = request.send().await?;
        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data).await?;
        Ok(paginated)
    }

    async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<FoundedDocument> {
        let model_id = params.model_id().as_ref().unwrap_or(self.config.model_id());
        let query = params.build_query(Some(model_id));
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let request = self
            .client
            .search(SearchParts::Index(&indexes))
            .pretty(true)
            .body(query);

        let request = match params.result().offset() > 0 {
            true => request.from(params.result().offset()),
            false => request.scroll(SCROLL_LIFETIME),
        };

        let response = request.send().await?;
        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data).await?;
        Ok(paginated)
    }

    async fn semantic_with_tokens(
        &self,
        params: &SemanticSearchWithTokensParams,
    ) -> PaginateResult<FoundedDocument> {
        let query = params.build_query(None);
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let response = self
            .client
            .search(SearchParts::Index(&indexes))
            .scroll(SCROLL_LIFETIME)
            .pretty(true)
            .body(query)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data).await?;
        Ok(paginated)
    }
}

#[async_trait::async_trait]
impl PaginateManager for OpenSearchStorage {
    async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        let response = self
            .client
            .clear_scroll(ClearScrollParts::ScrollId(&[session_id]))
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(())
    }

    async fn paginate(&self, params: &PaginateParams) -> PaginateResult<FoundedDocument> {
        let response = self
            .client
            .scroll(ScrollParts::ScrollId(params.scroll_id()))
            .pretty(true)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data).await?;
        Ok(paginated)
    }
}

#[cfg(test)]
mod test_osearch {
    use super::*;
    use crate::config::ServiceConfig;
    use crate::logger;

    const TEST_FOLDER_ID: &str = "test-common-folder";
    const TEST_DOCUMENTS_DATA: &[u8] =
        include_bytes!("../../../tests/resources/test-document.json");
    const TEST_FULLTEXT_DATA: &[u8] =
        include_bytes!("../../../tests/resources/fulltext-params.json");
    const TEST_RETRIEVE_DATA: &[u8] =
        include_bytes!("../../../tests/resources/retrieve-params.json");
    const TEST_SEMANTIC_DATA: &[u8] =
        include_bytes!("../../../tests/resources/semantic-params.json");

    #[tokio::test]
    async fn test_searcher_api() -> anyhow::Result<()> {
        let config = ServiceConfig::new()?;
        let config = config.storage().opensearch();
        let client = Arc::new(OpenSearchStorage::connect(config).await?);

        let _ = client.delete_index(TEST_FOLDER_ID).await;
        let _ = create_test_index(client.clone()).await;

        let documents = serde_json::from_slice::<Vec<Document>>(TEST_DOCUMENTS_DATA)?;
        for doc in documents.iter() {
            let result = client.create_document(TEST_FOLDER_ID, doc.clone()).await;
            assert!(result.is_ok());
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let retrieve_params = serde_json::from_slice::<RetrieveDocumentParams>(TEST_RETRIEVE_DATA)?;
        let result = client.retrieve(&retrieve_params).await;
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

    #[tokio::test]
    async fn test_documents_api() -> anyhow::Result<()> {
        let config = ServiceConfig::new()?;
        let config = config.storage().opensearch();
        let client = Arc::new(OpenSearchStorage::connect(config).await?);

        let _ = client.delete_index(TEST_FOLDER_ID).await;
        let _ = create_test_index(client.clone()).await;

        let documents = serde_json::from_slice::<Vec<Document>>(TEST_DOCUMENTS_DATA)?;
        for doc in documents.iter() {
            let result = client.create_document(TEST_FOLDER_ID, doc.clone()).await;
            if let Err(e) = result {
                return Err(e.into());
            }
            assert!(result.is_ok());

            let result = client.get_document(TEST_FOLDER_ID, doc.id()).await;
            assert!(result.is_ok());

            let loaded_doc = result?;
            assert_eq!(doc.id(), loaded_doc.id());
            assert_eq!(doc.content(), loaded_doc.content());

            client.delete_document(TEST_FOLDER_ID, doc.id()).await?;
            let result = client.get_document(TEST_FOLDER_ID, doc.id()).await;
            assert!(result.is_err());
        }

        let _ = client.delete_index(TEST_FOLDER_ID).await;

        Ok(())
    }

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

    async fn create_test_index(client: Arc<OpenSearchStorage>) -> anyhow::Result<Index> {
        let index = Index::builder()
            .id(TEST_FOLDER_ID.to_owned())
            .name(TEST_FOLDER_ID.to_owned())
            .path("".to_owned())
            .build()?;

        let _ = client.create_index(index.clone()).await?;
        Ok(index)
    }
}
