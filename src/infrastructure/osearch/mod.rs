pub mod config;
mod dto;
mod error;
mod extractor;
mod query;
mod schema;

use opensearch::auth::Credentials;
use opensearch::cat::CatIndicesParts;
use opensearch::cert::CertificateValidation;
use opensearch::http::headers::HeaderMap;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::{Method, Url};
use opensearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use opensearch::ingest::IngestPutPipelineParts;
use opensearch::OpenSearch;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::application::dto::params::{
    CreateIndexParams, FullTextSearchParams, HybridSearchParams, KnnIndexParams, PaginateParams,
    QueryBuilder, RetrieveDocumentParams, SemanticSearchParams,
};
use crate::application::dto::{Document, FoundedDocument, Index};
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
    async fn create_index(&self, params: &CreateIndexParams) -> StorageResult<String> {
        let id = params.id();
        let folder_schema =
            schema::create_document_schema(self.config.cluster(), params.knn().as_ref());
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

        Ok(params.id().to_owned())
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
        let offset = format!("{}*", self.config.username());
        let response = self
            .client
            .cat()
            .indices(CatIndicesParts::Index(&[&offset]))
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
            .filter(|it| !it.index().starts_with('.'))
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
    async fn create_document(&self, index: &str, doc: Document) -> StorageResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let response = self
            .client
            .create(opensearch::CreateParts::IndexId(index, &id))
            .pipeline(schema::INGEST_PIPELINE_NAME)
            .body(&doc)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(id)
    }

    async fn get_document(&self, index: &str, id: &str) -> StorageResult<Document> {
        let response = self
            .client
            .get(opensearch::GetParts::IndexId(index, id))
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
            .delete(opensearch::DeleteParts::IndexId(index, id))
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(())
    }

    async fn update_document(&self, index: &str, id: &str, doc: Document) -> StorageResult<()> {
        let response = self
            .client
            .update(opensearch::UpdateParts::IndexId(index, id))
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
    async fn retrieve(
        &self,
        ids: &str,
        params: &RetrieveDocumentParams,
    ) -> PaginateResult<FoundedDocument> {
        let query = params.build_query(None);
        let indexes = ids.split(',').collect::<Vec<&str>>();
        let search_parts = Self::build_search_parts(&indexes);
        let response = self
            .client
            .search(search_parts)
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
        let search_parts = Self::build_search_parts(&indexes);
        let request = self
            .client
            .search(search_parts)
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

    async fn hybrid(&self, params: &HybridSearchParams) -> PaginateResult<FoundedDocument> {
        let model_id = params
            .model_id()
            .as_ref()
            .unwrap_or(self.config.semantic().model_id());
        let query = params.build_query(Some(model_id));
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let search_parts = Self::build_search_parts(&indexes);
        let request = self.client.search(search_parts).pretty(true).body(query);

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
        let model_id = params
            .model_id()
            .as_ref()
            .unwrap_or(self.config.semantic().model_id());
        let query = params.build_query(Some(model_id));
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let search_parts = Self::build_search_parts(&indexes);
        let request = self.client.search(search_parts).pretty(true).body(query);

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
}

#[async_trait::async_trait]
impl PaginateManager for OpenSearchStorage {
    async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        let response = self
            .client
            .clear_scroll(opensearch::ClearScrollParts::ScrollId(&[session_id]))
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
            .scroll(opensearch::ScrollParts::ScrollId(params.scroll_id()))
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

impl OpenSearchStorage {
    pub async fn init_pipelines(&self, params: &KnnIndexParams) -> StorageResult<()> {
        let ingest_schema = schema::create_ingest_schema(self.config.semantic(), Some(params));
        let response = self
            .client
            .ingest()
            .put_pipeline(IngestPutPipelineParts::Id(schema::INGEST_PIPELINE_NAME))
            .body(ingest_schema)
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        let url = format!("/_search/pipeline/{}", schema::HYBRID_SEARCH_PIPELINE_NAME);
        let hs_schema = schema::create_hybrid_search_schema(self.config.semantic());
        let schema_bytes = serde_json::to_vec(&hs_schema)?;
        let response = self
            .client
            .transport()
            .send(
                Method::Put,
                &url,
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(&schema_bytes),
                None,
            )
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(())
    }

    fn build_search_parts<'a>(indexes: &'a [&'a str]) -> opensearch::SearchParts<'a> {
        match indexes.first() {
            Some(&"*") => opensearch::SearchParts::None,
            _ => opensearch::SearchParts::Index(indexes),
        }
    }
}

#[cfg(test)]
mod test_osearch {
    use super::*;
    use crate::application::dto::params::KnnIndexParams;
    use crate::config::ServiceConfig;

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

    #[tokio::test]
    async fn test_documents_api() -> anyhow::Result<()> {
        let config = ServiceConfig::new()?;
        let config = config.storage().opensearch();
        let client = Arc::new(OpenSearchStorage::connect(config).await?);

        let _ = client.delete_index(TEST_FOLDER_ID).await;
        let _ = create_test_index(client.clone()).await;

        let documents = serde_json::from_slice::<Vec<Document>>(TEST_DOCUMENTS_DATA)?;
        for doc in documents.iter() {
            let id = match client.create_document(TEST_FOLDER_ID, doc.clone()).await {
                Ok(id) => id,
                Err(err) => {
                    return Err(err.into());
                }
            };

            let result = client.get_document(TEST_FOLDER_ID, &id).await;
            assert!(result.is_ok());

            let loaded_doc = result?;
            assert_eq!(doc.content(), loaded_doc.content());

            client.delete_document(TEST_FOLDER_ID, &id).await?;
            let result = client.get_document(TEST_FOLDER_ID, &id).await;
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

    async fn create_test_index(client: Arc<OpenSearchStorage>) -> anyhow::Result<String> {
        let create_index = CreateIndexParams::builder()
            .id(TEST_FOLDER_ID.to_owned())
            .name(TEST_FOLDER_ID.to_owned())
            .path("".to_owned())
            .knn(Some(KnnIndexParams::default()))
            .build()
            .unwrap();

        let id = client.create_index(&create_index).await?;
        Ok(id)
    }
}
