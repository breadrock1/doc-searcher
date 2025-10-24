mod config;
mod dto;
mod error;
mod extractor;
mod query;
mod schema;
#[cfg(test)]
mod tests;

pub use config::OSearchConfig;

use anyhow::anyhow;
use opensearch::auth::Credentials;
use opensearch::cat::CatIndicesParts;
use opensearch::cert::CertificateValidation;
use opensearch::http::headers::HeaderMap;
use opensearch::http::request::JsonBody;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::{Method, Url};
use opensearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use opensearch::ingest::IngestPutPipelineParts;
use opensearch::OpenSearch;
use serde_derive::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::application::services::storage::{
    DocumentManager, DocumentSearcher, IndexManager, PaginateManager,
};
use crate::application::services::storage::{PaginateResult, StorageError, StorageResult};
use crate::application::structures::params::{
    CreateIndexParams, FullTextSearchParams, HybridSearchParams, KnnIndexParams, PaginateParams,
    RetrieveDocumentParams, SemanticSearchParams,
};
use crate::application::structures::{DocumentPart, FoundedDocument, Index, StoredDocumentPart};
use crate::infrastructure::osearch::config::OSearchKnnConfig;
use crate::infrastructure::osearch::dto::SourceDocument;
use crate::infrastructure::osearch::query::{QueryBuilder, QueryBuilderParams};
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

        tracing::debug!(address = config.address(), "connected to opensearch");
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
    #[tracing::instrument(skip(self), level = "info")]
    async fn create_index(&self, params: &CreateIndexParams) -> StorageResult<String> {
        let id = params.id();
        let knn_params = params.knn().as_ref();
        let cluster_config = self.config.cluster();
        let folder_schema = schema::create_document_schema(cluster_config, knn_params);
        let response = self
            .client
            .indices()
            .create(IndicesCreateParts::Index(id))
            .body(folder_schema)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(params.id().to_owned())
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn delete_index(&self, id: &str) -> StorageResult<()> {
        let response = self
            .client
            .indices()
            .delete(IndicesDeleteParts::Index(&[id]))
            .timeout("1m")
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn get_all_indexes(&self) -> StorageResult<Vec<Index>> {
        #[cfg(feature = "enable-multi-user")]
        let offset = format!("{}-*", self.config.username());
        #[cfg(feature = "enable-multi-user")]
        let response = self
            .client
            .cat()
            .indices(CatIndicesParts::Index(&[&offset]))
            .format("json")
            .send()
            .await?;

        // TODO: Remove this code after full implementation multi-user supporting
        #[cfg(not(feature = "enable-multi-user"))]
        let response = self
            .client
            .cat()
            .indices(CatIndicesParts::None)
            .format("json")
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
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

    #[tracing::instrument(skip(self), level = "info")]
    async fn get_index(&self, id: &str) -> StorageResult<Index> {
        let response = self
            .client
            .cat()
            .indices(CatIndicesParts::Index(&[id]))
            .format("json")
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

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
    #[tracing::instrument(skip(self), level = "info")]
    async fn store_document_parts(
        &self,
        index: &str,
        docs: &[DocumentPart],
    ) -> StorageResult<Vec<StoredDocumentPart>> {
        let mut operations: Vec<JsonBody<_>> = Vec::with_capacity(docs.len() * 2);
        let mut stored_documents = Vec::<StoredDocumentPart>::with_capacity(docs.len());

        for doc in docs {
            #[cfg(not(feature = "enable-unique-doc-id"))]
            let id = uuid::Uuid::new_v4().to_string();
            #[cfg(feature = "enable-unique-doc-id")]
            let id = Self::gen_unique_document_id(index, doc);

            stored_documents.push(StoredDocumentPart::new(
                id.clone(),
                doc.file_path().to_owned(),
            ));

            let header = json!({"index": {"_id": id}}).into();
            operations.push(header);

            let body = serde_json::to_value(doc)?.into();
            operations.push(body);
        }

        let response = self
            .client
            .bulk(opensearch::BulkParts::Index(index))
            .pipeline(schema::INGEST_PIPELINE_NAME)
            .body(operations)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(stored_documents)
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn get_document(&self, index: &str, id: &str) -> StorageResult<DocumentPart> {
        let response = self
            .client
            .get(opensearch::GetParts::IndexId(index, id))
            .pretty(true)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let document: DocumentPart = response.json::<SourceDocument>().await?.into();
        Ok(document)
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()> {
        let response = self
            .client
            .delete(opensearch::DeleteParts::IndexId(index, id))
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn update_document(
        &self,
        index: &str,
        id: &str,
        doc: &DocumentPart,
    ) -> StorageResult<()> {
        let doc_object =
            extractor::build_update_document_object(doc).map_err(StorageError::InternalError)?;

        // TODO: How update chunked_text and embeddings after updating content field automatically?
        let response = self
            .client
            .update(opensearch::UpdateParts::IndexId(index, id))
            .body(json!({"doc": doc_object}))
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl DocumentSearcher for OpenSearchStorage {
    #[tracing::instrument(skip(self), level = "info")]
    async fn retrieve(
        &self,
        ids: &str,
        params: &RetrieveDocumentParams,
    ) -> PaginateResult<FoundedDocument> {
        let query_params = QueryBuilderParams::from(params);
        let query = params.build_query(query_params);
        let indexes = ids.split(',').collect::<Vec<&str>>();
        let search_parts = Self::build_search_parts(&indexes);
        let request = self
            .client
            .search(search_parts)
            .pretty(true)
            .size(params.result().size());

        let request = match params.result().offset() > 0 {
            true => request.from(params.result().offset()),
            false => request.scroll(SCROLL_LIFETIME),
        };

        let response = request.body(query).send().await?;
        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data)?;
        Ok(paginated)
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<FoundedDocument> {
        let query_params = QueryBuilderParams::from(params);
        let query = params.build_query(query_params);
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
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data)?;
        Ok(paginated)
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn hybrid(&self, params: &HybridSearchParams) -> PaginateResult<FoundedDocument> {
        let model_id = params
            .model_id()
            .as_ref()
            .unwrap_or(self.config.semantic().model_id());

        let mut query_params = QueryBuilderParams::from(params);
        query_params.set_model_id_if_none(model_id);
        let query = params.build_query(query_params);
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let search_parts = Self::build_search_parts(&indexes);
        let request = self.client.search(search_parts).pretty(true).body(query);

        let request = match params.result().offset() > 0 {
            true => request.from(params.result().offset()),
            false => request.scroll(SCROLL_LIFETIME),
        };

        let response = request.send().await?;
        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data)?;
        Ok(paginated)
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<FoundedDocument> {
        let model_id = params
            .model_id()
            .as_ref()
            .unwrap_or(self.config.semantic().model_id());

        let mut query_params = QueryBuilderParams::from(params);
        query_params.set_model_id_if_none(model_id);
        let query = params.build_query(query_params);
        let indexes = params.indexes().split(',').collect::<Vec<&str>>();
        let search_parts = Self::build_search_parts(&indexes);
        let request = self.client.search(search_parts).pretty(true).body(query);

        let request = match params.result().offset() > 0 {
            true => request.from(params.result().offset()),
            false => request.scroll(SCROLL_LIFETIME),
        };

        let response = request.send().await?;
        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data)?;
        Ok(paginated)
    }
}

#[async_trait::async_trait]
impl PaginateManager for OpenSearchStorage {
    #[tracing::instrument(skip(self), level = "info")]
    async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        let response = self
            .client
            .clear_scroll(opensearch::ClearScrollParts::ScrollId(&[session_id]))
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self), level = "info")]
    async fn paginate(&self, params: &PaginateParams) -> PaginateResult<FoundedDocument> {
        let response = self
            .client
            .scroll(opensearch::ScrollParts::ScrollId(params.scroll_id()))
            .pretty(true)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let response_data = response.json::<Value>().await?;
        let paginated = extractor::extract_founded_docs(response_data)?;
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
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
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
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn load_ml_model(&self, config: &OSearchKnnConfig) -> StorageResult<()> {
        #[derive(Debug, Deserialize)]
        struct DeployModelTaskResponse {
            pub task_id: String,
            pub status: String,
        }

        #[derive(Debug, Deserialize)]
        struct DeployModelFetchResponse {
            model_id: Option<String>,
            state: String,
        }

        let schema_query = json!({
            "parameters": {
                "wait_for_completion": true
            }
        });

        let target_url = format!("/_plugins/_ml/models/{}/_load", config.model_id());
        let response = self
            .client
            .send(
                Method::Post,
                target_url.as_str(),
                HeaderMap::new(),
                None::<&String>,
                Some(schema_query.to_string()),
                None,
            )
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let deploy_task = response.json::<DeployModelTaskResponse>().await?;
        tracing::debug!(deploy_task=?deploy_task, "created deploy task");

        let mut await_task_completed = true;
        let target_url = format!("/_plugins/_ml/tasks/{}", deploy_task.task_id);
        while await_task_completed {
            let response = self
                .client
                .send(
                    Method::Get,
                    target_url.as_str(),
                    HeaderMap::new(),
                    None::<&String>,
                    Some(schema_query.to_string()),
                    None,
                )
                .await?;

            if !response.status_code().is_success() {
                let err = error::OSearchError::from_response(response).await;
                return Err(StorageError::from(err));
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let fetch_response = response.json::<DeployModelFetchResponse>().await?;
            tracing::debug!(fetch_response=?fetch_response, "fetched task status");
            await_task_completed = match fetch_response.state.as_str() {
                "FAILED" => {
                    let msg = "failed to deploy model";
                    return Err(StorageError::ServiceError(anyhow!(msg)));
                }
                "COMPLETED" => false,
                _ => true,
            };
        }

        Ok(())
    }

    fn build_search_parts<'a>(indexes: &'a [&'a str]) -> opensearch::SearchParts<'a> {
        match indexes.first() {
            Some(&"*") => opensearch::SearchParts::None,
            _ => opensearch::SearchParts::Index(indexes),
        }
    }

    #[cfg(feature = "enable-unique-doc-id")]
    pub fn gen_unique_document_id(index: &str, doc: &DocumentPart) -> String {
        let common_file_path = format!("{index}/{}/{}", doc.file_path(), doc.doc_part_id());
        let digest = md5::compute(&common_file_path);
        format!("{digest:x}")
    }
}
