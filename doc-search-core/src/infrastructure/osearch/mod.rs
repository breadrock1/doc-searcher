#[cfg(test)]
mod tests;

mod config;
pub use config::OSearchConfig;

mod dto;
mod error;
mod extractor;
mod query;
mod schema;

use anyhow::{Context, anyhow};
use metrics::{counter, histogram};
use opensearch::auth::Credentials;
use opensearch::cat::CatIndicesParts;
use opensearch::cert::CertificateValidation;
use opensearch::http::headers::HeaderMap;
use opensearch::http::request::JsonBody;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::{Method, Url};
use opensearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use opensearch::{DeleteByQueryParts, OpenSearch};
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::Instant;
use tracing::instrument;

#[cfg(feature = "enable-unique-doc-id")]
use crate::application::usecase::storage::gen_unique_document_id;

use crate::ServiceConnect;
use crate::domain::searcher::models::{Pagination, PaginationParams, SearchingParams};
use crate::domain::searcher::{IPaginator, ISearcher};
use crate::domain::searcher::{SearchError, SearchResult};
use crate::domain::storage::models::CreateIndexParams;
use crate::domain::storage::models::{AllDocumentParts, DocumentPart};
use crate::domain::storage::models::{StoredDocumentPartsInfo, StoredDocumentPartsInfoBuilder};
use crate::domain::storage::{IDocumentPartStorage, IIndexStorage};
use crate::domain::storage::{StorageError, StorageResult};
use crate::infrastructure::osearch::dto::RetrieveAllDocPartsQueryParamsBuilder;
use crate::infrastructure::osearch::dto::{FoundedDocumentInfo, IndexInformation, SourceDocument};
use crate::infrastructure::osearch::query::{QueryBuildHelper, build_search_query};
use crate::shared::kernel::{DocumentPartId, IndexId, LargeDocumentId, Tokens};

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

        tracing::info!(address=%config.address(), "connected to opensearch");
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
    #[instrument(level = "info", skip(self))]
    async fn create_index(&self, params: &CreateIndexParams) -> StorageResult<IndexId> {
        let index_id = &params.id;
        let knn_params = params.knn.as_ref();
        let folder_schema = schema::build_index_mappings(&self.config, knn_params);

        let start = Instant::now();
        let result = self
            .client
            .indices()
            .create(IndicesCreateParts::Index(index_id))
            .body(folder_schema)
            .send()
            .await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("create_index", start, is_success);
        let response = result?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
        }

        Ok(IndexId(index_id.to_owned()))
    }

    #[instrument(level = "info", skip(self))]
    async fn delete_index(&self, index_id: &IndexId) -> StorageResult<()> {
        let start = Instant::now();
        let result = self
            .client
            .indices()
            .delete(IndicesDeleteParts::Index(&[index_id.as_string()]))
            .timeout(EXECUTE_TIMEOUT)
            .send()
            .await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("delete_index", start, is_success);
        let response = result?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
        }

        Ok(())
    }

    #[instrument(level = "info", skip(self))]
    async fn get_index(&self, index_id: &IndexId) -> StorageResult<IndexId> {
        let start = Instant::now();
        let result = self
            .client
            .cat()
            .indices(CatIndicesParts::Index(&[index_id.as_string()]))
            .format(RESPONSE_FORMAT)
            .send()
            .await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("get_index", start, is_success);
        let response = result?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
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

    #[instrument(level = "info", skip_all)]
    async fn get_all_indexes(&self) -> StorageResult<Vec<IndexId>> {
        let start = Instant::now();
        let result = self
            .client
            .cat()
            .indices(CatIndicesParts::None)
            .format("json")
            .send()
            .await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("get_all_indexes", start, is_success);
        let response = result?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
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
    #[instrument(
        level = "info",
        skip_all,
        fields(
            index_id = index_id.0,
            document_parts_amount = all_doc_parts.len(),
        ),
    )]
    async fn store_document_parts(
        &self,
        index_id: &IndexId,
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

        let start = Instant::now();
        let result = self
            .client
            .bulk(opensearch::BulkParts::Index(index_id.as_string()))
            .body(operations)
            .send()
            .await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("bulk", start, is_success);
        let response = result?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
        }

        let first_doc_id = stored_doc_ids
            .first()
            .map(|it| DocumentPartId(it.clone()))
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

    #[instrument(level = "info", skip(self))]
    async fn get_document_parts(
        &self,
        index: &IndexId,
        large_doc_id: &LargeDocumentId,
    ) -> StorageResult<AllDocumentParts> {
        let query_params = RetrieveAllDocPartsQueryParamsBuilder::default()
            .large_doc_id(large_doc_id.to_string())
            .only_first_part(false)
            .with_sorting(true)
            .build()
            .context("failed to build query params")
            .map_err(StorageError::InternalError)?;

        let query = query_params.build_query(None);
        let indexes = index.as_string().split(',').collect::<Vec<&str>>();
        let search_parts = Self::build_search_parts(&indexes);

        let request = self.client.search(search_parts).pretty(true);

        let start = Instant::now();
        let result = request.body(query).send().await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("get_document_parts", start, is_success);
        let response = result?;
        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
        }

        let response_data = response.json::<Value>().await?;
        let all_doc_parts = extractor::extract_retrieved_document_parts(response_data)?;
        Ok(all_doc_parts)
    }

    #[instrument(level = "info", skip(self))]
    async fn get_document_part(
        &self,
        index: &IndexId,
        doc_part_id: &DocumentPartId,
    ) -> StorageResult<DocumentPart> {
        let start = Instant::now();
        let result = self
            .client
            .get(opensearch::GetParts::IndexId(
                index.as_string(),
                doc_part_id.as_string(),
            ))
            .pretty(true)
            .send()
            .await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("get_document_part", start, is_success);
        let response = result?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
        }

        let document: DocumentPart = response
            .json::<FoundedDocumentInfo>()
            .await?
            .try_into()
            .context("schema mismatching of founded document parts")
            .map_err(StorageError::InternalError)?;

        Ok(document)
    }

    #[instrument(level = "info", skip(self))]
    async fn delete_document_parts(
        &self,
        index: &IndexId,
        large_doc_id: &LargeDocumentId,
    ) -> StorageResult<()> {
        let query_params = RetrieveAllDocPartsQueryParamsBuilder::default()
            .large_doc_id(large_doc_id.to_string())
            .only_first_part(false)
            .with_sorting(false)
            .build()
            .context("failed to build query params")
            .map_err(StorageError::InternalError)?;

        let query = query_params.build_query(None);
        let indexes = index.as_string().split(',').collect::<Vec<&str>>();
        let start = Instant::now();
        let result = self
            .client
            .delete_by_query(DeleteByQueryParts::Index(&indexes))
            .body(query)
            .send()
            .await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("delete_document_parts", start, is_success);
        let response = result?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl ISearcher for OSearchClient {
    #[instrument(level = "info", skip(self))]
    async fn search(
        &self,
        params: SearchingParams,
        tokens: Option<Tokens>,
    ) -> SearchResult<Pagination> {
        let search_indexes = params.indexes.clone();
        let indexes = search_indexes
            .iter()
            .map(|it| it.as_str())
            .collect::<Vec<&str>>();

        let result_size = params.get_result().size;
        let result_offset = params.get_result().offset;
        let is_hybrid_search = params.is_hybrid_kind();

        let search_parts = Self::build_search_parts(&indexes);
        let query = build_search_query(params, tokens)?;
        let query_str = serde_json::to_string(&query).expect("should always serialize");
        tracing::debug!(query=?query_str, "search query");
        let request = self
            .client
            .search(search_parts)
            .pretty(true)
            .size(result_size);

        let request_builder = match result_offset > 0 {
            true => request.from(result_offset),
            false if is_hybrid_search => request.from(result_offset),
            false => request.scroll(SCROLL_LIFETIME),
        };

        let start = Instant::now();
        let search_result = request_builder
            .body(query)
            .send()
            .await
            .context("failed to send query result");
        let is_success = search_result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("search", start, is_success);
        let response = search_result.map_err(SearchError::InternalError)?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
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
    #[instrument(level = "info", skip(self))]
    async fn paginate(&self, params: PaginationParams) -> SearchResult<Pagination> {
        let start = Instant::now();
        let scroll_result = self
            .client
            .scroll(opensearch::ScrollParts::ScrollId(&params.scroll_id))
            .pretty(true)
            .send()
            .await
            .context("pagination failed");
        let is_success = scroll_result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("scroll", start, is_success);
        let response = scroll_result.map_err(SearchError::InternalError)?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(err.into());
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
    pub async fn init_pipelines(&self) -> StorageResult<()> {
        let url = format!("/_search/pipeline/{}", schema::HYBRID_SEARCH_PIPELINE_NAME);
        let schema_bytes = serde_json::to_vec(&schema::build_hybrid_search_schema())
            .context("response deserialization error")
            .map_err(StorageError::InternalError)?;

        let start = Instant::now();
        let result = self
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
            .await;
        let is_success = result
            .as_ref()
            .map(|r| r.status_code().is_success())
            .unwrap_or(false);
        Self::emit_call_metrics("put_search_pipeline", start, is_success);
        let response = result?;

        if !response.status_code().is_success() {
            let err = error::OSearchError::from_response(response).await;
            return Err(StorageError::from(err));
        }

        Ok(())
    }

    /// Records the outcome of a single outgoing OpenSearch call.
    ///
    /// `operation` is a stable name (e.g. "search", "bulk", "scroll") used to
    /// break down metrics; `is_success` mirrors `response.status_code().is_success()`.
    fn emit_call_metrics(operation: &str, start: Instant, is_success: bool) {
        counter!(
            "docsearch_opensearch_requests_total",
            "operation" => operation.to_owned(),
            "status" => is_success.to_string(),
        )
        .increment(1);
        histogram!(
            "docsearch_opensearch_request_duration_seconds",
            "operation" => operation.to_owned(),
        )
        .record(start.elapsed().as_secs_f64());
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
