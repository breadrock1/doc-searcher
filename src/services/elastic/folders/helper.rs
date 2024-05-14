use crate::errors::WebError;
use crate::forms::folder::{Folder, FolderType};
use crate::forms::schemas::document::DocumentSchema;
use crate::forms::schemas::preview::DocumentPreviewSchema;
use crate::forms::schemas::vector::DocumentVectorSchema;

use serde_json::Value;

pub(crate) fn extract_folder_stats(value: &Value) -> Result<Folder, WebError> {
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
        .map_err(|err| WebError::GetFolder(err.to_string()))?;

    Ok(folder)
}

pub(crate) fn create_folder_schema(schema_type: &FolderType) -> Value {
    match schema_type {
        FolderType::Document => serde_json::to_value(DocumentSchema::default()),
        FolderType::DocumentVector => serde_json::to_value(DocumentVectorSchema::default()),
        FolderType::DocumentPreview => serde_json::to_value(DocumentPreviewSchema::default()),
    }
    .unwrap()
}
