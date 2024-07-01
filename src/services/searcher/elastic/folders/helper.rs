use crate::errors::{WebError, WebErrorEntity, WebResult};
use crate::forms::folders::folder::{Folder, INFO_FOLDER_ID};
use crate::forms::folders::forms::{CreateFolderForm, FolderType};
use crate::forms::folders::info::InfoFolder;
use crate::forms::documents::schema::{DocumentSchema, DocumentVectorSchema};
use crate::forms::folders::schema::InfoFolderSchema;
use crate::forms::searcher::s_params::SearchParams;
use crate::services::searcher::elastic::context::ContextOptions;
use crate::services::searcher::elastic::searcher::helper as s_helper;
use crate::services::searcher::elastic::searcher::extractor::SearcherTrait;

use elasticsearch::http::response::Response;
use elasticsearch::{Elasticsearch, GetParts, IndexParts};
use serde_json::{json, Value};
use std::collections::HashMap;

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

// TODO: Need refactoring this method
pub(crate) async fn filter_folders(elastic: &Elasticsearch, ctx_opts: &ContextOptions, folders: Vec<Folder>, show_all: bool) -> WebResult<Vec<Folder>> {
    let indexes = &[INFO_FOLDER_ID];

    let mut s_params = SearchParams::default();
    s_params.set_show_all(show_all);

    let body_value = InfoFolder::build_query(&s_params, ctx_opts).await;
    let response = s_helper::send_search_request(elastic, &s_params, &body_value, indexes).await?;
    let info_folders = s_helper::extract_elastic_response::<InfoFolder>(response).await;

    let mut info_folders_map: HashMap<&str, InfoFolder > = HashMap::new();
    info_folders
        .get_founded()
        .iter()
        .for_each(|info| {
            let id = info.get_id();
            let info_cln = info.to_owned();
            info_folders_map.insert(id, info_cln);
        });

    let mut common_folders_info: Vec<Folder> = Vec::new();
    for mut folder in folders {
        let info_folder_opt = info_folders_map.get(folder.get_index());
        if info_folder_opt.is_none() {
            if show_all {
                common_folders_info.push(folder);
            }
            continue;
        }

        let info_folder = info_folder_opt.unwrap();
        let folder_mut = &mut folder;
        folder_mut.set_name(info_folder.get_name());

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

pub(crate) async fn load_info_doc(elastic: &Elasticsearch, folder: &mut Folder) -> WebResult<()> {
    let response = elastic
        .get(GetParts::IndexId(INFO_FOLDER_ID, folder.get_index()))
        .refresh(true)
        .send()
        .await
        .map_err(WebError::from)?;

    let info_folder = response.json::<InfoFolder>().await.map_err(WebError::from)?;
    Ok(folder.set_name(info_folder.get_name()))
}