use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::forms::folders::folder::{Folder, INFO_FOLDER_ID};
use crate::forms::folders::forms::{CreateFolderForm, FolderType};
use crate::forms::folders::info::InfoFolder;
use crate::forms::schemas::document::DocumentSchema;
use crate::forms::schemas::embeddings::DocumentVectorSchema;
use crate::forms::schemas::folder::InfoFolderSchema;
use crate::forms::searcher::s_params::SearchParams;
use crate::services::searcher::elastic::context::ContextOptions;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::elastic::searcher::helper as s_helper;

use elasticsearch::http::response::Response;
use elasticsearch::{Elasticsearch, IndexParts};
use serde_json::{json, Value};
use std::collections::HashMap;
use elasticsearch::http::Method;

pub(super) async fn create_index(
    elastic: &Elasticsearch,
    folder_form: &CreateFolderForm,
) -> Result<Response, WebError> {
    let folder_id = folder_form.get_id();
    let folder_schema = create_folder_schema(folder_form.get_schema());
    elastic
        .index(IndexParts::Index(folder_id))
        .body(&json!({folder_id: folder_schema}))
        .send()
        .await
        .map_err(WebError::from)
}

pub(super) fn extract_folder_stats(value: &Value) -> Result<Folder, WebError> {
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

    let docs_count = docs_count - docs_deleted - 1;

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

pub(super) fn create_folder_schema(schema_type: &FolderType) -> Value {
    match schema_type {
        FolderType::InfoFolder => serde_json::to_value(InfoFolderSchema::default()),
        FolderType::Vectors => serde_json::to_value(DocumentVectorSchema::default()),
        _ => serde_json::to_value(DocumentSchema::default()),
    }
    .unwrap()
}

pub(crate) async fn filter_folders(elastic: &Elasticsearch, ctx_opts: &ContextOptions, folders: Vec<Folder>) -> WebResult<Vec<Folder>> {
    let s_params = SearchParams::default();
    let info_folders = s_helper::search::<InfoFolder>(
        &elastic, 
        &s_params, 
        ctx_opts, 
        &["info-folder"]
    )
    .await?;
    
    let mut info_folders_map: HashMap<&str, InfoFolder > = HashMap::new();
    info_folders
        .get_founded()
        .iter()
        .for_each(|info| {
            let id = info.get_id();
            let info_cln = info.to_owned();
            info_folders_map.insert(id, info_cln);
        });

    let mut new_vec: Vec<Folder> = Vec::new();
    for mut test in folders {
        let tes = &mut test;
        if filter_system_folders(tes, &mut info_folders_map) {
            new_vec.push(test);
        }
    }
    
    Ok(new_vec)
}

pub(crate) fn filter_system_folders(folder: &mut Folder, info_folders: &mut HashMap<&str, InfoFolder>) -> bool {
    let fold_id = folder.get_index();
    match info_folders.get(fold_id) {
        None => false,
        Some(info_folder) => {
            if info_folder.is_system() {
                return false;
            }

            let name = info_folder.get_name();
            folder.set_name(Some(name.to_string()));
            true
        }
    }
}

pub(crate) async fn del_from_info_folder(elastic: &Elasticsearch, folder_id: &str) -> WebResult<Successful> {
    let s_path = format!("/{}/_doc/{}", INFO_FOLDER_ID, folder_id);
    let response = helper::send_elrequest(&elastic, Method::Delete, None, s_path.as_str()).await?;
    helper::parse_elastic_response(response).await
}
