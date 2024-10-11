use crate::errors::{Successful, WebErrorEntity, WebError, WebResult};
use crate::searcher::elastic::helper as s_helper;
use crate::searcher::models::SearchParams;
use crate::searcher::SearcherTrait;
use crate::storage::DocumentsTrait;
use crate::storage::elastic::schema::{DocumentSchema, DocumentVectorSchema};
use crate::storage::elastic::schema::InfoFolderSchema;
use crate::storage::elastic::store::StoreTrait;
use crate::storage::models::Document;
use crate::storage::models::{Folder, INFO_FOLDER_ID, InfoFolder};
use crate::storage::forms::{CreateFolderForm, FolderType};

use elasticsearch::http::response::Response;
use elasticsearch::params::Refresh;
use elasticsearch::{BulkParts, Elasticsearch, GetParts, IndexParts, UpdateParts};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

pub async fn store_object<T>(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    folder_id: &str,
    doc_form: &T,
) -> WebResult<Successful>
where
    T: DocumentsTrait + StoreTrait<T> + serde::Serialize + Sized,
{
    let response = elastic
        .index(IndexParts::IndexId(folder_id, doc_form.get_doc_id()))
        .refresh(Refresh::True)
        .timeout("1m")
        .body(&doc_form)
        .send()
        .await
        .map_err(WebError::from)?;

    s_helper::parse_elastic_response(response).await
}

// TODO: Combine those methods to common
pub async fn store_objects<T>(
    elastic: &RwLockReadGuard<'_, Elasticsearch>,
    folder_id: &str,
    doc_form: &T,
) -> WebResult<Successful>
where
    T: DocumentsTrait + StoreTrait<T> + serde::Serialize + Sized,
{
    let body = T::create_body(doc_form).await;
    let response = elastic
        .bulk(BulkParts::Index(folder_id))
        .refresh(Refresh::True)
        .timeout("1m")
        .body(body)
        .send()
        .await
        .map_err(WebError::from)?;

    s_helper::parse_elastic_response(response).await
}

pub async fn extract_document<'de, T: serde::Deserialize<'de>>(
    response: Response,
) -> Result<T, WebError> {
    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"_source"];
    T::deserialize(document_json.to_owned()).map_err(WebError::from)
}

pub async fn update_document(
    es_cxt: Arc<RwLock<Elasticsearch>>,
    folder_id: &str,
    doc_form: &Document,
) -> WebResult<Successful> {
    let elastic = es_cxt.write().await;
    let doc_id = doc_form.get_doc_id();
    let response = elastic
        .update(UpdateParts::IndexId(folder_id, doc_id))
        .body(&json!({
            "doc": doc_form,
        }))
        .send()
        .await?;

    s_helper::parse_elastic_response(response).await
}

pub async fn create_index(elastic: &Elasticsearch, folder_form: &CreateFolderForm) -> WebResult<Response> {
    let folder_id = folder_form.folder_id();
    let folder_type = folder_form.folder_type();
    let folder_schema = create_folder_schema(folder_type);
    elastic
        .index(IndexParts::Index(folder_id))
        .body(&json!({folder_id: folder_schema}))
        .send()
        .await
        .map_err(WebError::from)
}

pub fn extract_folder_stats(value: &Value) -> Result<Folder, WebError> {
    let indices = &value[&"indices"];
    let folder_id = indices.as_object().unwrap().keys().next().unwrap();

    let index_value = &indices[folder_id.as_str()];
    let health = &index_value[&"health"].as_str().unwrap();
    let status = &index_value[&"status"].as_str().unwrap();
    let uuid = &index_value[&"uuid"].as_str().unwrap();

    let primaries = &value[&"_all"][&"primaries"];
    let docs_count = &primaries[&"docs"][&"count"].as_i64().unwrap();
    let docs_deleted = &primaries[&"docs"][&"deleted"].as_i64().unwrap();
    let store_size = &primaries[&"store"][&"size_in_bytes"].as_i64().unwrap();
    let pri_store_size = &primaries[&"store"][&"total_data_set_size_in_bytes"]
        .as_i64()
        .unwrap();

    let folder = Folder::builder()
        .name(None)
        .health(health.to_string())
        .status(status.to_string())
        .index(folder_id.to_owned())
        .uuid(uuid.to_string())
        .docs_count(Some(docs_count.to_string()))
        .docs_deleted(Some(docs_deleted.to_string()))
        .store_size(Some(store_size.to_string()))
        .pri_store_size(Some(pri_store_size.to_string()))
        .pri(None)
        .rep(None)
        .build()
        .map_err(|err| {
            let entity = WebErrorEntity::new(err.to_string());
            WebError::GetFolder(entity)
        })?;

    Ok(folder)
}

pub fn create_folder_schema(schema_type: &FolderType) -> Value {
    match schema_type {
        FolderType::InfoFolder => serde_json::to_value(InfoFolderSchema::default()),
        FolderType::Vectors => serde_json::to_value(DocumentVectorSchema::default()),
        _ => serde_json::to_value(DocumentSchema::default()),
    }
    .unwrap()
}

// TODO: Need refactoring this method
pub async fn filter_folders(
    elastic: &Elasticsearch,
    folders: Vec<Folder>,
    show_all: bool,
) -> WebResult<Vec<Folder>> {
    let indexes = &[INFO_FOLDER_ID];

    let mut s_params = SearchParams::default();
    s_params.set_show_all(show_all);

    let body_value = InfoFolder::build_query(&s_params).await;
    let response = s_helper::send_search_request(elastic, &s_params, &body_value, indexes).await?;
    let info_folders = s_helper::extract_elastic_response::<InfoFolder>(response).await;

    let mut info_folders_map: HashMap<&str, InfoFolder> = HashMap::new();
    info_folders.get_founded().iter().for_each(|info| {
        let id = info.index();
        let info_cln = info.to_owned();
        info_folders_map.insert(id, info_cln);
    });

    let mut common_folders_info: Vec<Folder> = Vec::new();
    for mut folder in folders {
        let folder_id = folder.index().as_str();
        let info_folder_opt = info_folders_map.get(folder_id);
        if info_folder_opt.is_none() {
            if show_all {
                common_folders_info.push(folder);
            }
            continue;
        }

        let info_folder = info_folder_opt.unwrap();
        let folder_mut = &mut folder;
        let info_folder_name = info_folder.name();
        folder_mut.set_name(info_folder_name);

        if show_all {
            common_folders_info.push(folder);
            continue;
        }

        if !info_folder.is_system() {
            common_folders_info.push(folder);
        }
    }

    Ok(common_folders_info)
}

pub async fn load_info_doc(elastic: &Elasticsearch, folder: &mut Folder) -> WebResult<()> {
    let response = elastic
        .get(GetParts::IndexId(INFO_FOLDER_ID, folder.index()))
        .refresh(true)
        .send()
        .await
        .map_err(WebError::from)?;

    let info_folder = response
        .json::<InfoFolder>()
        .await
        .map_err(WebError::from)?;

    folder.set_name(info_folder.name());
    Ok(())
}
