use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::storage::elastic::documents::helper as d_helper;
use crate::storage::elastic::documents::retrieve::Retrieve;
use crate::storage::elastic::folders::schema;
use crate::storage::elastic::EsCxt;
use crate::storage::errors::StorageResult;
use crate::storage::forms::{CreateFolderForm, RetrieveParams};
use crate::storage::models::INFO_FOLDER_ID;
use crate::storage::models::{DocumentVectors, Folder, InfoFolder};

use elasticsearch::indices::IndicesDeleteParts;
use elasticsearch::params::Refresh;
use elasticsearch::{DeleteParts, GetParts, IndexParts};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn create_index(es_cxt: EsCxt, form: &CreateFolderForm) -> StorageResult<Successful> {
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

pub async fn load_folder_info(
    es_cxt: EsCxt,
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

    match d_helper::extract_document::<InfoFolder>(response).await {
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

pub async fn delete_folder_info(es_cxt: EsCxt, index: &str) -> StorageResult<Successful> {
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
