use serde::Deserialize;
use serde_json::{json, Value};

use crate::application::services::storage::{PaginateResult, StorageResult};
use crate::application::structures::{DocumentPart, FoundedDocument, PaginatedBuilder};
use crate::infrastructure::osearch::dto::SourceDocument;

#[tracing::instrument(level = "info")]
pub fn extract_founded_docs(common_object: Value) -> PaginateResult<FoundedDocument> {
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

pub fn build_update_document_object(doc: &DocumentPart) -> anyhow::Result<Value> {
    let mut doc_value = json!({
        "file_name": doc.file_name(),
        "file_path": doc.file_path(),
        "file_size": doc.file_size(),
        "created_at": doc.created_at(),
        "modified_at": doc.modified_at(),
    });

    if let Some(content) = doc.content().as_ref() {
        doc_value["content"] = json!(content);
    };

    if let Some(chunked_text) = doc.chunked_text().as_ref() {
        doc_value["chunked_text"] = json!(chunked_text);
    }

    if let Some(embeddings) = doc.embeddings().as_ref() {
        doc_value["embeddings"] = json!(embeddings);
    }

    Ok(doc_value)
}
