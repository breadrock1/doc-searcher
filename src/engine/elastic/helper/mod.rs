pub(super) mod converter;
pub mod extractor;

use elasticsearch::http::response::Response;
use elasticsearch::indices::IndicesDeleteParts;
use elasticsearch::params::Refresh;
use elasticsearch::{DeleteParts, Elasticsearch, GetParts, IndexParts};
use serde::ser::Error;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::engine::elastic::ops::retrieve::Retrieve;
use crate::engine::elastic::ops::store::StoreTrait;
use crate::engine::elastic::ops::update::UpdateTrait;
use crate::engine::elastic::{schema, ElasticClient};
use crate::engine::error::{StorageError, StorageResult};
use crate::engine::form::{CreateFolderForm, RetrieveParams};
use crate::engine::model::{
    Document, DocumentVectors, DocumentsTrait, Folder, FolderType, InfoFolder, INFO_FOLDER_ID,
};
use crate::errors::Successful;

pub async fn create_index(
    es_cxt: Arc<RwLock<Elasticsearch>>,
    form: &CreateFolderForm,
) -> StorageResult<Successful> {
    let folder_id = form.folder_id();
    let folder_schema = schema::build_schema_by_folder_type(form.folder_type())?;

    let elastic = es_cxt.write().await;
    let response = elastic
        .index(IndexParts::Index(folder_id))
        .body(&json!({folder_id: folder_schema}))
        .send()
        .await?;

    let response = ElasticClient::extract_response_msg(response).await?;
    Ok(response)
}

pub async fn delete_index(
    es_cxt: Arc<RwLock<Elasticsearch>>,
    index: &str,
) -> StorageResult<Successful> {
    let elastic = es_cxt.write().await;
    let response = elastic
        .indices()
        .delete(IndicesDeleteParts::Index(&[index]))
        .timeout("1m")
        .send()
        .await?;

    let response = ElasticClient::extract_response_msg(response).await?;
    Ok(response)
}

pub async fn load_folder_info(
    es_cxt: Arc<RwLock<Elasticsearch>>,
    index: &str,
    folder: &mut Folder,
) -> StorageResult<()> {
    let elastic = es_cxt.read().await;
    let response = elastic
        .get(GetParts::IndexId(INFO_FOLDER_ID, index))
        .refresh(true)
        .send()
        .await?
        .error_for_status_code()?;

    match extract_document::<InfoFolder>(response).await {
        Err(err) => {
            tracing::warn!("failed to load folder name: {err:#?}");
            Err(err)
        }
        Ok(info_folder) => {
            let folder_uman_name = info_folder.name().to_owned();
            folder.set_name(Some(folder_uman_name));
            Ok(())
        }
    }
}

pub async fn delete_folder_info(
    es_cxt: Arc<RwLock<Elasticsearch>>,
    index: &str,
) -> StorageResult<Successful> {
    let elastic = es_cxt.write().await;
    let response = elastic
        .delete(DeleteParts::IndexId(INFO_FOLDER_ID, index))
        .refresh(Refresh::True)
        .timeout("1m")
        .send()
        .await?;

    let response = ElasticClient::extract_response_msg(response).await?;
    Ok(response)
}

pub async fn filter_folders(
    es_cxt: Arc<RwLock<Elasticsearch>>,
    folders: &[Folder],
    show_all: bool,
) -> StorageResult<Vec<Folder>> {
    let mut params = RetrieveParams::default();
    params.set_is_show_all(show_all);

    let indexes = &[INFO_FOLDER_ID];
    let results = (params.result_size(), params.result_offset());
    let query = DocumentVectors::build_retrieve_query(&params).await;
    let response = ElasticClient::search_request(es_cxt, &query, None, indexes, results).await?;

    let value = response.json::<Value>().await?;
    let Some(founded) = &value[&"hits"][&"hits"].as_array() else {
        tracing::warn!("not founded info-folder items");
        return Ok(Vec::default());
    };

    let info_folders = founded
        .iter()
        .filter_map(|it| InfoFolder::extract_from_response(it).ok())
        .map(|it| (it.index().to_owned(), it))
        .collect::<HashMap<String, InfoFolder>>();

    let filtered = folders
        .iter()
        .filter(|folder| filter_info_folder(folder, &info_folders, show_all))
        .map(Folder::to_owned)
        .map(|folder| set_folders_name(folder, &info_folders))
        .collect::<Vec<Folder>>();

    Ok(filtered)
}

fn filter_info_folder(
    folder: &Folder,
    info_folders: &HashMap<String, InfoFolder>,
    show_all: bool,
) -> bool {
    let Some(info_folder) = info_folders.get(folder.index()) else {
        return show_all;
    };

    if show_all || !info_folder.is_system() {
        return true;
    }

    false
}

fn set_folders_name(mut folder: Folder, info_folders: &HashMap<String, InfoFolder>) -> Folder {
    let folder_id = folder.index();
    if let Some(info_folder) = info_folders.get(folder_id) {
        folder.set_name(Some(info_folder.name().to_owned()));
    }

    folder
}

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
        let err = serde_json::Error::custom(msg);
        return Err(StorageError::SerdeError(err));
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
        es: Arc<RwLock<Elasticsearch>>,
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
        es: Arc<RwLock<Elasticsearch>>,
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
        es: Arc<RwLock<Elasticsearch>>,
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
