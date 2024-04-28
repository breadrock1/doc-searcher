use std::collections::HashMap;
use tokio::sync::RwLockReadGuard;
use wrappers::document::Document;
use wrappers::search_params::SearchParams;

pub(crate) fn filter_founded_documents(
    map: RwLockReadGuard<HashMap<String, Document>>,
    bucket_id: &str,
    s_params: &SearchParams,
) -> Vec<Document> {
    map.values()
        .filter(|doc| doc.bucket_uuid.eq(bucket_id))
        .filter(|doc| doc.content.contains(&s_params.query))
        .cloned()
        .collect::<Vec<Document>>()
}
