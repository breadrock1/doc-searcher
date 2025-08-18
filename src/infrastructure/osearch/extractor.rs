use serde::Deserialize;
use serde_json::Value;

use crate::application::dto::{FoundedDocument, PaginatedBuilder};
use crate::application::services::storage::{PaginateResult, StorageResult};
use crate::infrastructure::osearch::dto::SourceDocument;

pub async fn extract_founded_docs(common_object: Value) -> PaginateResult<FoundedDocument> {
    let scroll_id = common_object[&"_scroll_id"].as_str().map(String::from);
    let founded_hits = common_object[&"hits"][&"hits"].as_array();
    let Some(hits) = founded_hits else {
        tracing::warn!("returned empty array of founded documents");
        let paginated_result = PaginatedBuilder::default()
            .founded(Vec::default())
            .scroll_id(scroll_id)
            .build()
            .unwrap();

        return Ok(paginated_result);
    };

    let documents = hits
        .iter()
        .filter_map(|it| extract_founded_document(it).ok())
        .collect::<Vec<FoundedDocument>>();

    let documents = PaginatedBuilder::default()
        .scroll_id(scroll_id)
        .founded(documents)
        .build()
        .unwrap();

    Ok(documents)
}

pub fn extract_founded_document(value: &Value) -> StorageResult<FoundedDocument> {
    let src_document = SourceDocument::deserialize(value)?;
    let document: FoundedDocument = src_document.into();
    Ok(document)
}
