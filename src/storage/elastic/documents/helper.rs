use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::storage::elastic::documents::retrieve::Retrieve;
use crate::storage::elastic::documents::store::StoreTrait;
use crate::storage::elastic::documents::update::UpdateTrait;
use crate::storage::elastic::EsCxt;
use crate::storage::errors::{StorageError, StorageResult};
use crate::storage::forms::RetrieveParams;
use crate::storage::models::{Document, DocumentVectors};
use crate::storage::models::{DocumentsTrait, FolderType};

use elasticsearch::http::response::Response;
use serde::Deserialize;
use serde_json::Value;

pub async fn extract_document<'de, T>(response: Response) -> StorageResult<T>
where
    T: DocumentsTrait + serde::Deserialize<'de>,
{
    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    let document = T::deserialize(document_json.to_owned())?;
    Ok(document)
}

pub async fn extract_all_documents<'de, T>(response: Response) -> StorageResult<Vec<Value>>
where
    T: Retrieve<'de, T> + DocumentsTrait + serde::Serialize + serde::Deserialize<'de>,
{
    let value = response.json::<Value>().await?;
    let founded_arr = &value[&"hits"][&"hits"].as_array();
    let Some(values) = founded_arr else {
        let msg = "returned empty data to get all documents";
        tracing::warn!(details = msg, "failed to extract documents");
        return Err(StorageError::SerdeError(msg.to_string()));
    };

    let documents = values
        .iter()
        .filter_map(|val| match T::extract_from_response(val) {
            Ok(doc) => serde_json::to_value(doc).ok(),
            Err(err) => {
                tracing::error!(err=?err, "failed to extract documents");
                None
            }
        })
        .collect::<Vec<Value>>();

    Ok(documents)
}

impl FolderType {
    pub async fn get_document(&self, response: Response) -> StorageResult<Value> {
        let common_object = response.json::<Value>().await?;
        let document_json = &common_object[&"_source"];

        match self {
            FolderType::Vectors => {
                let document = DocumentVectors::deserialize(document_json.to_owned())?;
                let value = serde_json::to_value(document)?;
                Ok(value)
            }
            _ => {
                let document = Document::deserialize(document_json.to_owned())?;
                let value = serde_json::to_value(document)?;
                Ok(value)
            }
        }
    }

    pub async fn get_all_documents(
        &self,
        es: EsCxt,
        indexes: &[&str],
        params: &RetrieveParams,
    ) -> StorageResult<Vec<Value>> {
        let results = (params.result_size(), params.result_offset());
        match self {
            FolderType::Vectors => {
                let query = DocumentVectors::build_retrieve_query(params).await;
                let response =
                    ElasticClient::search_request(es, &query, None, indexes, results).await?;
                let value = extract_all_documents::<DocumentVectors>(response).await?;
                Ok(value)
            }
            _ => {
                let query = Document::build_retrieve_query(params).await;
                let response =
                    ElasticClient::search_request(es, &query, None, indexes, results).await?;
                let value = extract_all_documents::<Document>(response).await?;
                Ok(value)
            }
        }
    }

    pub async fn create_document(
        &self,
        es: EsCxt,
        index: &str,
        doc: &Document,
    ) -> StorageResult<Successful> {
        match self {
            FolderType::Vectors => {
                let docs_vec = DocumentVectors::from(doc);
                DocumentVectors::store_all(es, index, &docs_vec).await
            }
            _ => {
                let mut doc_cln = doc.to_owned();
                doc_cln.exclude_tokens();
                Document::store_all(es, index, &doc_cln).await
            }
        }
    }

    pub async fn update_document(
        &self,
        es: EsCxt,
        index: &str,
        doc: &Value,
    ) -> StorageResult<Successful> {
        match self {
            FolderType::Vectors => {
                let doc = DocumentVectors::deserialize(doc)?;
                DocumentVectors::update(es, index, &doc).await
            }
            _ => {
                let doc = Document::deserialize(doc)?;
                Document::update(es, index, &doc).await
            }
        }
    }
}
