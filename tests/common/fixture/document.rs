use doc_search_core::domain::storage::models::{LargeDocument, LargeDocumentBuilder};
use serde_derive::Deserialize;

const LARGE_DOCUMENTS_PATH: &[u8] = include_bytes!("../../resources/test-documents.json");
const LARGE_DOCUMENT_CONTENT: &str = "there is some huge content about current project";
const LARGE_DOCUMENT_CREATED_TIMESTAMP: i64 = 12375128745;

#[derive(Clone, Deserialize)]
struct LargeDocumentObject {
    file_name: String,
    file_path: String,
    file_size: u32,
    created_at: i64,
    modified_at: i64,
    content: String,
}

impl TryFrom<LargeDocumentObject> for LargeDocument {
    type Error = anyhow::Error;

    fn try_from(doc: LargeDocumentObject) -> Result<Self, Self::Error> {
        let large_document = LargeDocumentBuilder::default()
            .file_name(doc.file_name)
            .file_path(doc.file_path)
            .file_size(doc.file_size)
            .created_at(doc.created_at)
            .modified_at(doc.modified_at)
            .content(doc.content)
            .build()?;

        Ok(large_document)
    }
}

pub fn build_large_document() -> LargeDocument {
    LargeDocumentBuilder::default()
        .file_name("test-document.docx".to_string())
        .file_path("./test-document.docx".to_string())
        .file_size(1024)
        .created_at(LARGE_DOCUMENT_CREATED_TIMESTAMP)
        .modified_at(LARGE_DOCUMENT_CREATED_TIMESTAMP)
        .content(LARGE_DOCUMENT_CONTENT.to_string())
        .build()
        .unwrap()
}

#[allow(dead_code)]
pub fn build_real_large_document() -> anyhow::Result<LargeDocument> {
    let real_documents = serde_json::from_slice::<Vec<LargeDocumentObject>>(LARGE_DOCUMENTS_PATH)?;
    let first_large_document = real_documents
        .first()
        .cloned()
        .expect("should have document at least one project");

    let document = first_large_document.try_into()?;
    Ok(document)
}

pub fn build_real_large_documents() -> anyhow::Result<Vec<LargeDocument>> {
    let real_documents = serde_json::from_slice::<Vec<LargeDocumentObject>>(LARGE_DOCUMENTS_PATH)?;
    let large_documents = real_documents
        .into_iter()
        .map(LargeDocument::try_from)
        .filter_map(Result::ok)
        .collect();

    Ok(large_documents)
}
