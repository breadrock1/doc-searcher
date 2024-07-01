extern crate doc_search;

use doc_search::services::config;
use doc_search::services::searcher::elastic;
use doc_search::services::searcher::service::FolderService;
use doc_search::forms::folders::forms::{CreateFolderForm, FolderType};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let s_config = config::init_service_config()?;
    let es_client = elastic::build_searcher_service(&s_config)?;

    let info_folder_form = CreateFolderForm::builder()
        .folder_id("info-folder".to_string())
        .folder_name("Info Folder".to_string())
        .folder_type(FolderType::InfoFolder)
        .create_into_watcher(false)
        .location("/".to_string())
        .user_id("admin".to_string())
        .is_system(true)
        .build()?;

    let common_folder_form = CreateFolderForm::builder()
        .folder_id("common-folder".to_string())
        .folder_name("Common Folder".to_string())
        .folder_type(FolderType::Document)
        .create_into_watcher(false)
        .location("/indexer/watcher".to_string())
        .user_id("admin".to_string())
        .is_system(false)
        .build()?;

    let common_vec_folder_form = CreateFolderForm::builder()
        .folder_id("common-folder-vector".to_string())
        .folder_name("Common Folder Vector".to_string())
        .folder_type(FolderType::Vectors)
        .create_into_watcher(false)
        .location("/indexer/watcher".to_string())
        .user_id("admin".to_string())
        .is_system(true)
        .build()?;

    for folder_form in vec![info_folder_form, common_folder_form, common_vec_folder_form] {
        match es_client.create_folder(&folder_form).await {
            Ok(resp) => log::info!("done: {}", resp.message),
            Err(err) => log::warn!("failed: {:?}", err),
        }
    }

    Ok(())
}

