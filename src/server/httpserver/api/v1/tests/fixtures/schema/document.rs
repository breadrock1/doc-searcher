use serde::Deserialize;

use crate::server::httpserver::api::v1::schema::DocumentPartSchema;

const DOCUMENT_DATA: &[u8] = include_bytes!("../../resources/document_part.json");

pub fn document_part_schema() -> DocumentPartSchema {
    load_document_part::<DocumentPartSchema>()
}

fn load_document_part<T: Deserialize<'static>>() -> T {
    serde_json::from_slice(DOCUMENT_DATA).expect("failed to parse document")
}
