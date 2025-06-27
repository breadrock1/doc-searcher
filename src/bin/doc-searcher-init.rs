use doc_search::{config, ServiceConnect};
use doc_search::application::services::storage::IndexManager;
use doc_search::application::dto::Index;
use doc_search::infrastructure::osearch::OpenSearchStorage;

const COMMON_FOLDER_ID: &str = "common-folder";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let s_config = config::ServiceConfig::new()?;
    let osearch_config = s_config.storage().open_search();
    let osearch_client = OpenSearchStorage::connect(osearch_config).await?;

    let common_folder_form = Index::builder()
        .id(COMMON_FOLDER_ID.to_string())
        .name(COMMON_FOLDER_ID.to_string())
        .path("./".to_string())
        .build()?;

    let common_vec_folder_id = format!("{COMMON_FOLDER_ID}-vector");
    let common_vec_folder_form = Index::builder()
        .id(common_vec_folder_id.clone())
        .name(common_vec_folder_id)
        .path("/".to_string())
        .build()?;

    for index in vec![common_folder_form, common_vec_folder_form] {
        let index_id = index.id().clone();
        let result = osearch_client.create_index(index).await;
        match result {
            Ok(_) => tracing::info!(index=index_id, "index created successful"),
            Err(err) => tracing::error!(index=index_id, err=?err, "failed ot create index"),
        }
    }

    Ok(())
}
