use crate::storage::models::Folder;

use serde::de::Error;
use serde_json::Value;

#[async_trait::async_trait]
pub trait FromElasticResponse<'de, T>
where
    T: serde::Deserialize<'de>,
{
    async fn from_value(value: Value) -> Result<T, serde_json::Error>;
}

#[async_trait::async_trait]
impl FromElasticResponse<'_, Folder> for Folder {
    async fn from_value(value: Value) -> Result<Folder, serde_json::Error> {
        let indices = &value[&"indices"];
        let indices_keys = indices.as_object().and_then(|it| it.keys().next());

        let Some(folder_id) = indices_keys else {
            let msg = "empty elastic search response";
            tracing::error!("{msg}");

            return Err(serde_json::Error::custom(msg));
        };

        let index_value = &indices[folder_id];
        let health = &index_value[&"health"].as_str().unwrap_or_default();
        let status = &index_value[&"status"].as_str().unwrap_or_default();
        let uuid = &index_value[&"uuid"].as_str().unwrap_or_default();

        let primaries = &value[&"_all"][&"primaries"];
        let docs_count = &primaries[&"docs"][&"count"].as_i64();
        let docs_deleted = &primaries[&"docs"][&"deleted"].as_i64();
        let store_size = &primaries[&"store"][&"size_in_bytes"].to_string();
        let pri_store = &primaries[&"store"][&"total_data_set_size_in_bytes"].to_string();

        Folder::builder()
            .name(None)
            .health(health.to_string())
            .status(status.to_string())
            .index(folder_id.to_owned())
            .uuid(uuid.to_string())
            .docs_count(docs_count.to_owned())
            .docs_deleted(docs_deleted.to_owned())
            .store_size(Some(store_size.to_owned()))
            .pri_store_size(Some(pri_store.to_owned()))
            .pri(None)
            .rep(None)
            .build()
            .map_err(|err| serde_json::Error::custom(err.to_string()))
    }
}
