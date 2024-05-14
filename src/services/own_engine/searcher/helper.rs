use crate::forms::documents::document::Document;
use crate::forms::s_params::SearchParams;
use std::collections::HashMap;
use tokio::sync::RwLockReadGuard;

pub(crate) fn filter_founded_documents(
    map: &RwLockReadGuard<HashMap<String, Document>>,
    bucket_id: &str,
    s_params: &SearchParams,
) -> Vec<Document> {
    let query = s_params.get_query();
    map.values()
        .filter(|doc| doc.get_folder_id().eq(bucket_id))
        .filter(|doc| doc.get_content().contains(query))
        .cloned()
        .collect::<Vec<Document>>()
}
