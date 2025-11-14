mod config;
pub use config::OSearchConfig;

mod error;
mod schema;
mod query;

#[cfg(test)]
mod tests;
mod dto;

use opensearch::auth::Credentials;
use opensearch::cat::CatIndicesParts;
use opensearch::cert::CertificateValidation;
use opensearch::http::request::JsonBody;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::Url;
use opensearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use opensearch::OpenSearch;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::core::storage::domain::{CreateIndexParams, DocumentPart, Index, StoredDocumentPart};
use crate::core::storage::domain::IDocumentStorage;
use crate::core::storage::domain::IIndexStorage;
use crate::core::storage::domain::{StorageError, StorageResult};
use crate::core::storage::infrastructure::osearch::dto::{OSearchIndex, SourceDocument};
use crate::ServiceConnect;

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
impl IIndexStorage for OpenSearchStorage {
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

    async fn get_all_indexes(&self) -> StorageResult<Vec<Index>> {
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
            .json::<Vec<OSearchIndex>>()
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
impl IDocumentStorage for OpenSearchStorage {
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

    async fn update_document(
        &self,
        index: &str,
        id: &str,
        doc: &DocumentPart,
    ) -> StorageResult<()> {
        let doc_object = build_update_document_object(doc).map_err(StorageError::InternalError)?;

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


}

pub fn build_update_document_object(doc: &DocumentPart) -> anyhow::Result<Value> {
    let mut doc_value = json!({
        "file_name": doc.file_name(),
        "file_path": doc.file_path(),
        "file_size": doc.file_size(),
        "created_at": doc.created_at(),
        "modified_at": doc.modified_at(),
    });

    if let Some(content) = doc.content().as_ref() {
        doc_value["content"] = json!(content);
    };

    if let Some(chunked_text) = doc.chunked_text().as_ref() {
        doc_value["chunked_text"] = json!(chunked_text);
    }

    if let Some(embeddings) = doc.embeddings().as_ref() {
        doc_value["embeddings"] = json!(embeddings);
    }

    Ok(doc_value)
}
