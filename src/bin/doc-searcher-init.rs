extern crate doc_search;

use doc_search::engine::elastic::ElasticClient;
use doc_search::engine::form::CreateFolderForm;
use doc_search::engine::model::{FolderType, DEFAULT_FOLDER_ID, INFO_FOLDER_ID};
use doc_search::engine::FolderService;
use doc_search::{config, ServiceConnect};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let s_config = config::ServiceConfig::new()?;
    let es_client = ElasticClient::connect(s_config.elastic()).await?;

    let info_folder_form = CreateFolderForm::builder()
        .folder_id(INFO_FOLDER_ID.to_string())
        .folder_name(INFO_FOLDER_ID.to_string())
        .folder_type(FolderType::InfoFolder)
        .create_into_watcher(false)
        .location("/".to_string())
        .user_id("admin".to_string())
        .is_system(true)
        .build()?;

    let common_folder_form = CreateFolderForm::builder()
        .folder_id(DEFAULT_FOLDER_ID.to_string())
        .folder_name(DEFAULT_FOLDER_ID.to_string())
        .folder_type(FolderType::Document)
        .create_into_watcher(false)
        .location("/indexer/watcher".to_string())
        .user_id("admin".to_string())
        .is_system(false)
        .build()?;

    let common_vec_folder_id = format!("{DEFAULT_FOLDER_ID}-vector");
    let common_vec_folder_form = CreateFolderForm::builder()
        .folder_id(common_vec_folder_id.clone())
        .folder_name(common_vec_folder_id)
        .folder_type(FolderType::Vectors)
        .create_into_watcher(false)
        .location("/indexer/watcher".to_string())
        .user_id("admin".to_string())
        .is_system(true)
        .build()?;

    for folder_form in vec![info_folder_form, common_folder_form, common_vec_folder_form] {
        match es_client.create_folder(&folder_form).await {
            Ok(resp) => tracing::info!("done: {resp:?}"),
            Err(err) => tracing::warn!("failed: {err:#?}"),
        }
    }

    Ok(())
}
