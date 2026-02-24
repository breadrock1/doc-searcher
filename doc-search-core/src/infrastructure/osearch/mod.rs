#[cfg(test)]
mod tests;

mod config;
mod dto;
mod error;
mod extractor;
mod query;
mod schema;

pub use config::OSearchConfig;

use anyhow::{Context, anyhow};
use opensearch::auth::Credentials;
use opensearch::cat::CatIndicesParts;
use opensearch::cert::CertificateValidation;
use opensearch::http::headers::HeaderMap;
use opensearch::http::request::JsonBody;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::{Method, Url};
use opensearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use opensearch::ingest::IngestPutPipelineParts;
use opensearch::{DeleteByQueryParts, OpenSearch};
use serde_derive::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

#[cfg(feature = "enable-unique-doc-id")]
use crate::application::usecase::storage::gen_unique_document_id;

use crate::ServiceConnect;
use crate::domain::searcher::models::{
    Pagination, PaginationParams, SearchKindParams, SearchingParams,
};
use crate::domain::searcher::{IPaginator, ISearcher};
use crate::domain::searcher::{SearchError, SearchResult};
use crate::domain::storage::models::{AllDocumentParts, DocumentPart};
use crate::domain::storage::models::{CreateIndexParams, IndexId, KnnIndexParams};
use crate::domain::storage::models::{StoredDocumentPartsInfo, StoredDocumentPartsInfoBuilder};
use crate::domain::storage::{IDocumentPartStorage, IIndexStorage};
use crate::domain::storage::{StorageError, StorageResult};
use crate::infrastructure::osearch::config::OSearchKnnConfig;
use crate::infrastructure::osearch::dto::RetrieveAllDocPartsQueryParamsBuilder;
use crate::infrastructure::osearch::dto::{FoundedDocumentInfo, IndexInformation, SourceDocument};
use crate::infrastructure::osearch::query::{QueryBuildHelper, build_search_query};

const SCROLL_LIFETIME: &str = "5m";
const EXECUTE_TIMEOUT: &str = "1m";
const RESPONSE_FORMAT: &str = "json";

#[derive(Clone)]
pub struct OSearchClient {
    config: OSearchConfig,
    client: Arc<OpenSearch>,
}

#[async_trait::async_trait]
impl ServiceConnect for OSearchClient {
    type Config = OSearchConfig;
    type Client = OSearchClient;
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

        tracing::debug!(address=%config.address(), "connected to opensearch");
        let client = OpenSearch::new(transport);
        let arc_client = Arc::new(client);
        Ok(OSearchClient {
            config: config.clone(),
            client: arc_client,
        })
    }
}

#[async_trait::async_trait]
impl IIndexStorage for OSearchClient {
    async fn create_index(&self, params: &CreateIndexParams) -> StorageResult<String> {
        let index_id = &params.id;
        let knn_params = params.knn.as_ref();
        let folder_schema = schema::build_index_mappings(&self.config, knn_params);

        let response = self
            .client
            .indices()
            .create(IndicesCreateParts::Index(index_id))
            .body(folder_schema)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(index_id.to_owned())
    }

    async fn delete_index(&self, index_id: &str) -> StorageResult<()> {
        let response = self
            .client
            .indices()
            .delete(IndicesDeleteParts::Index(&[index_id]))
            .timeout(EXECUTE_TIMEOUT)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(())
    }

    async fn get_index(&self, index_id: &str) -> StorageResult<IndexId> {
        let response = self
            .client
            .cat()
            .indices(CatIndicesParts::Index(&[index_id]))
            .format(RESPONSE_FORMAT)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let indexes = response
            .json::<Vec<IndexInformation>>()
            .await?
            .into_iter()
            .map(IndexInformation::into)
            .collect::<Vec<IndexId>>();

        let Some(index) = indexes.first() else {
            let err = anyhow::Error::msg("there is no index with such name");
            return Err(StorageError::IndexNotFound(err));
        };

        Ok(index.to_owned())
    }

    async fn get_all_indexes(&self) -> StorageResult<Vec<IndexId>> {
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
            .json::<Vec<IndexInformation>>()
            .await?
            .into_iter()
            .filter(|it| !it.index().starts_with('.'))
            .map(IndexInformation::into)
            .collect::<Vec<IndexId>>();

        Ok(indexes)
    }
}

#[async_trait::async_trait]
impl IDocumentPartStorage for OSearchClient {
    async fn store_document_parts(
        &self,
        index_id: &str,
        all_doc_parts: AllDocumentParts,
    ) -> StorageResult<StoredDocumentPartsInfo> {
        let doc_parts_amount = all_doc_parts.len();
        let large_doc_id = all_doc_parts
            .first()
            .as_ref()
            .map(|it| it.large_doc_id.clone())
            .ok_or(anyhow!("missing large document id to store"))
            .map_err(StorageError::InternalError)?;

        let mut stored_doc_ids = Vec::with_capacity(doc_parts_amount);
        let mut operations: Vec<JsonBody<Value>> = Vec::with_capacity(doc_parts_amount * 2);

        for doc in all_doc_parts.into_iter() {
            #[cfg(not(feature = "enable-unique-doc-id"))]
            let id = uuid::Uuid::new_v4().to_string();
            #[cfg(feature = "enable-unique-doc-id")]
            let id = gen_unique_document_id(index_id, &doc.large_doc_id, doc.doc_part_id);

            let doc_header = json!({"index": {"_id": id}}).into();
            operations.push(doc_header);

            let srd_doc: SourceDocument = doc.try_into()?;
            let doc_body = serde_json::to_value(srd_doc)
                .context("failed to serialize document to json")
                .map_err(StorageError::ValidationError)?
                .into();

            stored_doc_ids.push(id);
            operations.push(doc_body);
        }

        let response = self
            .client
            .bulk(opensearch::BulkParts::Index(index_id))
            .pipeline(schema::INGEST_PIPELINE_NAME)
            .body(operations)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let first_doc_id = stored_doc_ids
            .first()
            .ok_or(anyhow!("there is no any stored id's"))
            .map_err(StorageError::InternalError)?;

        let stored_doc_info = StoredDocumentPartsInfoBuilder::default()
            .large_doc_id(large_doc_id)
            .first_part_id(first_doc_id.clone())
            .doc_parts_amount(doc_parts_amount)
            .build()
            .context("failed to build stored document info")
            .map_err(StorageError::InternalError)?;

        Ok(stored_doc_info)
    }

    async fn get_document_parts(
        &self,
        index: &str,
        large_doc_id: &str,
    ) -> StorageResult<AllDocumentParts> {
        let query_params = RetrieveAllDocPartsQueryParamsBuilder::default()
            .large_doc_id(large_doc_id.to_string())
            .only_first_part(false)
            .with_sorting(true)
            .build()
            .context("failed to build query params")
            .map_err(StorageError::InternalError)?;

        let query = query_params.build_query();
        let indexes = index.split(',').collect::<Vec<&str>>();
        let search_parts = Self::build_search_parts(&indexes);

        let request = self.client.search(search_parts).pretty(true);

        let response = request.body(query).send().await?;
        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let response_data = response.json::<Value>().await?;
        let all_doc_parts = extractor::extract_retrieved_document_parts(response_data)?;
        Ok(all_doc_parts)
    }

    async fn get_document_part(
        &self,
        index: &str,
        doc_part_id: &str,
    ) -> StorageResult<DocumentPart> {
        let response = self
            .client
            .get(opensearch::GetParts::IndexId(index, doc_part_id))
            .pretty(true)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        let document: DocumentPart = response
            .json::<FoundedDocumentInfo>()
            .await?
            .try_into()
            .context("schema mismatching of founded document parts")
            .map_err(StorageError::InternalError)?;

        Ok(document)
    }

    async fn delete_document_parts(&self, index: &str, large_doc_id: &str) -> StorageResult<()> {
        let query_params = RetrieveAllDocPartsQueryParamsBuilder::default()
            .large_doc_id(large_doc_id.to_string())
            .only_first_part(false)
            .with_sorting(false)
            .build()
            .context("failed to build query params")
            .map_err(StorageError::InternalError)?;

        let query = query_params.build_query();
        let indexes = index.split(',').collect::<Vec<&str>>();
        let response = self
            .client
            .delete_by_query(DeleteByQueryParts::Index(&indexes))
            .body(query)
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
impl ISearcher for OSearchClient {
    async fn search(&self, params: &SearchingParams) -> SearchResult<Pagination> {
        let indexes = params
            .get_indexes()
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        let search_parts = Self::build_search_parts(&indexes);
        let query = build_search_query(params, self.config.semantic())?;
        let request = self
            .client
            .search(search_parts)
            .pretty(true)
            .size(params.get_result().size);

        let request_builder = match params.get_result().offset > 0 {
            true => request.from(params.get_result().offset),
            false => match params.get_kind() {
                SearchKindParams::Hybrid(_) => request.from(params.get_result().offset),
                _ => request.scroll(SCROLL_LIFETIME),
            },
        };

        let response = request_builder
            .body(query)
            .send()
            .await
            .context("failed to send query result")
            .map_err(SearchError::InternalError)?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(SearchError::InternalError(anyhow!(err)));
        }

        let response_data = response
            .json::<Value>()
            .await
            .context("failed to parse search result")
            .map_err(SearchError::InternalError)?;

        let paginated = extractor::extract_founded_document_parts(response_data)?;
        Ok(paginated)
    }
}

#[async_trait::async_trait]
impl IPaginator for OSearchClient {
    async fn paginate(&self, params: &PaginationParams) -> SearchResult<Pagination> {
        let response = self
            .client
            .scroll(opensearch::ScrollParts::ScrollId(&params.scroll_id))
            .pretty(true)
            .send()
            .await
            .context("pagination failed")
            .map_err(SearchError::InternalError)?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(SearchError::InternalError(anyhow!(err)));
        };

        let response_data = response
            .json::<Value>()
            .await
            .context("pagination error response")
            .map_err(SearchError::InternalError)?;

        let paginated = extractor::extract_founded_document_parts(response_data)?;
        Ok(paginated)
    }
}

impl OSearchClient {
    pub async fn update_cluster_settings(&self) -> StorageResult<()> {
        let cluster_settings = json!({
            "persistent": {
                "plugins.ml_commons.only_run_on_ml_node": false,
                "plugins.ml_commons.model_auto_redeploy.enable": true
              }
        });

        let response = self
            .client
            .cluster()
            .put_settings()
            .body(cluster_settings)
            .send()
            .await?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(())
    }

    pub async fn init_pipelines(&self, params: &KnnIndexParams) -> StorageResult<()> {
        let ingest_schema = schema::builder_ingest_schema(&self.config, Some(params));
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
        let hs_schema = schema::build_hybrid_search_schema(self.config.semantic());
        let schema_bytes = serde_json::to_vec(&hs_schema)
            .context("response deserialization error")
            .map_err(StorageError::InternalError)?;

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
        tracing::debug!(?deploy_task, "created deploy task");

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
            tracing::debug!(?fetch_response, "fetched task status");
            await_task_completed = match fetch_response.state.as_str() {
                "FAILED" => {
                    let msg = "failed to deploy model";
                    return Err(StorageError::InternalError(anyhow!(msg)));
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
        let common_file_path = format!("{index}/{}/{}", doc.file_path, doc.doc_part_id);
        let digest = md5::compute(&common_file_path);
        format!("{digest:x}")
    }
}
