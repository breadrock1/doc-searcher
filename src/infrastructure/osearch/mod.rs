pub mod config;
mod error;
mod query;
mod schema;
mod dto;

use opensearch::auth::Credentials;
use opensearch::cat::CatIndicesParts;
use opensearch::cert::CertificateValidation;
use opensearch::http::transport::SingleNodeConnectionPool;
use opensearch::http::transport::TransportBuilder;
use opensearch::http::Url;
use opensearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use opensearch::{CreateParts, OpenSearch};
use opensearch::{
    ClearScrollParts, DeleteParts, GetParts, ScrollParts, SearchParts,
    UpdateParts,
};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::application::dto::Paginated;
use crate::application::dto::{Document, Index, SemanticSearchWithTokensParams};
use crate::application::dto::{
    FullTextSearchParams, PaginateParams, QueryBuilder, RetrieveDocumentParams,
    SemanticSearchParams,
};
use crate::application::services::storage::error::{PaginateResult, StorageError, StorageResult};
use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::infrastructure::osearch::config::OSearchConfig;
use crate::ServiceConnect;

const ALL_INDEXES_ALIAS: &str = "*";
const CAT_INDICES_URL: &str = "/_cat/indices?format=json";

#[derive(Clone)]
pub struct OpenSearchStorage {
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
        Ok(OpenSearchStorage { client: arc_client })
    }
}

#[async_trait::async_trait]
impl IndexManager for OpenSearchStorage {
    async fn create_index(&self, index: Index) -> StorageResult<Index> {
        let id = index.id();
        let folder_schema = schema::create_document_schema();
        let response = self.client
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
        let response = self.client
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
            .body(&doc)
            .timeout("1m")
            .send()
            .await?;

        if !response.status_code().is_success() {
            return Err(error::OpenSearchError::from_response(response).await);
        }

        Ok(())
    }

    async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()> {
        let response = self
            .client
            .delete(DeleteParts::IndexId(index, id))
            .timeout("1m")
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

        let document = response.json::<Document>().await?;
        Ok(document)
    }

    async fn update_document(&self, index: &str, doc: Document) -> StorageResult<()> {
        let response = self.client
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
    async fn retrieve(
        &self,
        params: &RetrieveDocumentParams,
    ) -> StorageResult<Paginated<Vec<Document>>> {
        let query = params.build_query();
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let response = self
            .client
            .search(SearchParts::Index(&indexes))
            .allow_no_indices(true)
            .pretty(true)
            .scroll("1m")
            .from(params.result().offset())
            .size(params.result().size())
            .body(query)
            .send()
            .await?
            .error_for_status_code()?
            .json::<Value>()
            .await?;

        let result = query::extract_founded_docs(response).await?;
        Ok(result)
    }

    async fn fulltext(
        &self,
        params: &FullTextSearchParams,
    ) -> StorageResult<Paginated<Vec<Document>>> {
        let query = params.build_query();
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let response = self
            .client
            .search(SearchParts::Index(&indexes))
            .allow_no_indices(true)
            .pretty(true)
            .scroll("1m")
            .from(params.result().offset())
            .size(params.result().size())
            .body(query)
            .send()
            .await?
            .error_for_status_code()?
            .json::<Value>()
            .await?;

        let result = query::extract_founded_docs(response).await?;
        Ok(result)
    }

    async fn semantic(
        &self,
        params: &SemanticSearchParams,
    ) -> StorageResult<Paginated<Vec<Document>>> {
        let query = params.build_query();
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let response = self
            .client
            .search(SearchParts::Index(&indexes))
            .allow_no_indices(true)
            .pretty(true)
            .scroll("1m")
            .size(params.result().size())
            .body(query)
            .send()
            .await?
            .error_for_status_code()?
            .json::<Value>()
            .await?;

        let result = query::extract_founded_docs(response).await?;
        Ok(result)
    }

    async fn semantic_with_tokens(
        &self,
        params: &SemanticSearchWithTokensParams,
    ) -> PaginateResult<Document> {
        let query = params.build_query();
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let response = self
            .client
            .search(SearchParts::Index(&indexes))
            .allow_no_indices(true)
            .pretty(true)
            .scroll("1m")
            .size(params.result().size())
            .body(query)
            .send()
            .await?
            .error_for_status_code()?
            .json::<Value>()
            .await?;

        let result = query::extract_founded_docs(response).await?;
        Ok(result)
    }
}

#[async_trait::async_trait]
impl PaginateManager for OpenSearchStorage {
    async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        self.client
            .clear_scroll(ClearScrollParts::ScrollId(&[session_id]))
            .send()
            .await?
            .error_for_status_code()?;

        Ok(())
    }

    async fn paginate(&self, params: &PaginateParams) -> StorageResult<Paginated<Vec<Document>>> {
        let response = self
            .client
            .scroll(ScrollParts::ScrollId(params.scroll_id()))
            .pretty(true)
            .send()
            .await?
            .error_for_status_code()?
            .json::<Value>()
            .await?;

        let paginated = query::extract_founded_docs(response).await?;
        Ok(paginated)
    }
}
