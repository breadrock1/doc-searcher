use anyhow::Context;
use serde::Deserialize;
use serde_json::Value;

use crate::domain::searcher::models::{FoundedDocument, Pagination, PaginationBuilder};
use crate::domain::searcher::{SearchError, SearchResult};
use crate::domain::storage::StorageResult;
use crate::domain::storage::models::{AllDocumentParts, DocumentPart};
use crate::infrastructure::osearch::dto::FoundedDocumentInfo;
use crate::infrastructure::osearch::error::{OSearchError, OSearchResult};

pub fn extract_retrieved_document_parts(object: Value) -> StorageResult<AllDocumentParts> {
    let founded_hits = object[&"hits"][&"hits"].as_array();
    let Some(hits) = founded_hits else {
        tracing::warn!("returned empty array of founded documents");
        return Ok(Vec::default());
    };

    let document_parts = hits
        .iter()
        .filter_map(|it| extract_document(it).ok())
        .collect::<Vec<DocumentPart>>();

    Ok(document_parts)
}

pub fn extract_founded_document_parts(object: Value) -> SearchResult<Pagination> {
    let scroll_id = object[&"_scroll_id"].as_str().map(String::from);
    let founded_hits = object[&"hits"][&"hits"].as_array();
    let Some(hits) = founded_hits else {
        tracing::warn!("returned empty array of founded documents");
        let paginated_result = PaginationBuilder::default()
            .founded(Vec::default())
            .scroll_id(scroll_id)
            .build()
            .map_err(anyhow::Error::from)
            .map_err(SearchError::InternalError)?;

        return Ok(paginated_result);
    };

    let documents = hits
        .iter()
        .filter_map(|it| extract_document(it).ok())
        .collect::<Vec<FoundedDocument>>();

    let documents = PaginationBuilder::default()
        .scroll_id(scroll_id)
        .founded(documents)
        .build()
        .map_err(anyhow::Error::from)
        .map_err(SearchError::InternalError)?;

    Ok(documents)
}

fn extract_document<T>(value: &Value) -> OSearchResult<T>
where
    T: TryFrom<FoundedDocumentInfo, Error = OSearchError>,
{
    let founded_doc_info = FoundedDocumentInfo::deserialize(value)
        .context("failed to deserialize founded document")
        .map_err(OSearchError::ExecutionError)?;

    T::try_from(founded_doc_info)
}
