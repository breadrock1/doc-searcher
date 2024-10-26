use crate::searcher::forms::DocumentType;
use crate::searcher::models::Paginated;
use crate::storage::models::{Document, DocumentVectors, DocumentsTrait};

use serde_json::Value;
use std::collections::HashMap;

pub fn to_unified_paginated(
    mut paginated: Paginated<Vec<Document>>,
    doc_type: &DocumentType,
) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.scroll_id();
    let converted = paginated
        .founded_mut()
        .iter()
        .flat_map(|doc| doc_type.document_to_value(doc))
        .collect::<Vec<Value>>();

    Paginated::new_with_opt_id(converted, scroll_id)
}

pub fn vec_to_value(mut paginated: Paginated<Vec<DocumentVectors>>) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.scroll_id();
    let converted = paginated
        .founded_mut()
        .iter()
        .flat_map(serde_json::to_value)
        .collect::<Vec<Value>>();

    Paginated::new_with_opt_id(converted, scroll_id)
}

pub fn vec_to_grouped_value(paginated: Paginated<Vec<DocumentVectors>>) -> Paginated<Vec<Value>> {
    let scroll_id = paginated.scroll_id();
    let converted = group_document_chunks(paginated.founded());
    let values = serde_json::to_value(converted).unwrap();
    Paginated::new_with_opt_id(vec![values], scroll_id)
}

fn group_document_chunks(documents: &[DocumentVectors]) -> HashMap<String, Vec<DocumentVectors>> {
    let mut grouped_documents: HashMap<String, Vec<DocumentVectors>> = HashMap::new();
    documents.iter().for_each(|doc| {
        grouped_documents
            .entry(doc.get_doc_id().to_string())
            .or_default()
            .push(doc.to_owned())
    });

    grouped_documents
}
