use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::storage::elastic::retrieve::Retrieve;
use crate::storage::elastic::schema::{DocumentSchema, DocumentVectorSchema, InfoFolderSchema};
use crate::storage::elastic::EsCxt;
use crate::storage::errors::{StorageError, StorageResult};
use crate::storage::forms::{CreateFolderForm, RetrieveParams};
use crate::storage::models::{DocumentVectors, INFO_FOLDER_ID};
use crate::storage::models::{DocumentsTrait, Folder, FolderType, InfoFolder};

use elasticsearch::http::response::Response;
use elasticsearch::indices::IndicesDeleteParts;
use elasticsearch::params::Refresh;
use elasticsearch::{DeleteParts, GetParts, IndexParts};
use elschema::ElasticSchema;
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn extract_document<'de, T>(response: Response) -> StorageResult<T>
where
    T: DocumentsTrait + serde::Deserialize<'de>,
{
    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    let document = T::deserialize(document_json.to_owned())?;
    Ok(document)
}

pub async fn extract_from_response<'de, T>(response: Response) -> StorageResult<Vec<Value>>
where
    T: Retrieve<'de, T> + DocumentsTrait + serde::Serialize + serde::Deserialize<'de>,
{
    let value = response.json::<Value>().await?;
    let founded_arr = &value[&"hits"][&"hits"].as_array();
    let Some(values) = founded_arr else {
        let msg = "returned empty data to get all documents";
        tracing::warn!(msg);
        return Err(StorageError::SerdeError(msg.to_string()));
    };

    let documents = values
        .iter()
        .filter_map(|val| match T::extract_from_response(val) {
            Ok(doc) => serde_json::to_value(doc).ok(),
            Err(err) => {
                tracing::error!("failed to extract documents: {err:#?}");
                None
            }
        })
        .collect::<Vec<Value>>();

    Ok(documents)
}

pub async fn load_from_folder_info(es_cxt: EsCxt, index: &str) -> StorageResult<InfoFolder> {
    let elastic = es_cxt.read().await;
    let response = elastic
        .get(GetParts::IndexId(INFO_FOLDER_ID, index))
        .refresh(true)
        .send()
        .await?;

    extract_document::<InfoFolder>(response).await
}

pub async fn delete_from_folder_info(es_cxt: EsCxt, index: &str) -> StorageResult<Successful> {
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

pub async fn create_index(es_cxt: EsCxt, form: &CreateFolderForm) -> StorageResult<Successful> {
    let folder_id = form.folder_id();
    let folder_schema = build_schema_by_folder_type(form.folder_type())?;

    let elastic = es_cxt.write().await;
    let response = elastic
        .index(IndexParts::Index(folder_id))
        .body(&json!({folder_id: folder_schema}))
        .send()
        .await?;

    let response = ElasticClient::extract_response_msg(response).await?;
    Ok(response)
}

pub async fn delete_index(es_cxt: EsCxt, index: &str) -> StorageResult<Successful> {
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

pub async fn delete_document(
    es_cxt: EsCxt,
    index: &str,
    doc_id: &str,
) -> StorageResult<Successful> {
    let elastic = es_cxt.read().await;
    let response = elastic
        .delete(DeleteParts::IndexId(index, doc_id))
        .refresh(Refresh::True)
        .timeout("1m")
        .send()
        .await?;

    let response = ElasticClient::extract_response_msg(response).await?;
    Ok(response)
}

pub fn build_schema_by_folder_type(schema_type: &FolderType) -> StorageResult<Value> {
    match schema_type {
        FolderType::InfoFolder => {
            let schema = InfoFolderSchema::build();
            serde_json::to_value(schema)
        }
        FolderType::Vectors => {
            let schema = DocumentVectorSchema::build();
            serde_json::to_value(schema)
        }
        _ => {
            let schema = DocumentSchema::build();
            serde_json::to_value(schema)
        }
    }
    .map_err(StorageError::from)
}

pub async fn filter_folders(
    es_cxt: EsCxt,
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
