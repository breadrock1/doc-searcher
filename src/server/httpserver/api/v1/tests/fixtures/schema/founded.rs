use serde::Deserialize;

use crate::server::httpserver::api::v1::schema::FoundedDocumentPartSchema;

const FOUNDED_DOCUMENT_DATA: &[u8] = include_bytes!("../../resources/founded_document.json");

pub fn founded_document_schema() -> FoundedDocumentPartSchema {
    load_founded_document::<FoundedDocumentPartSchema>()
}

fn load_founded_document<T: Deserialize<'static>>() -> T {
    serde_json::from_slice(FOUNDED_DOCUMENT_DATA).expect("failed to parse document")
}
