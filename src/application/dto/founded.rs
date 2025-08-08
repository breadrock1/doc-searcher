use derive_builder::Builder;
use getset::Getters;
use serde_derive::Serialize;

use crate::application::dto::Document;

#[derive(Builder, Clone, Getters, Serialize)]
#[getset(get = "pub")]
pub struct FoundedDocument {
    id: String,
    folder_id: String,
    document: Document,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<f64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    highlight: Vec<String>,
}

impl FoundedDocument {
    pub fn builder() -> FoundedDocumentBuilder {
        FoundedDocumentBuilder::default()
    }
}

impl TryFrom<crate::domain::FoundedDocument> for FoundedDocument {
    type Error = FoundedDocumentBuilderError;

    fn try_from(founded_doc: crate::domain::FoundedDocument) -> Result<Self, Self::Error> {
        let highlight = founded_doc.highlight;
        let document = Document::try_from(founded_doc.document).unwrap();
        let result = FoundedDocument::builder()
            .id(founded_doc.id)
            .folder_id(founded_doc.folder_id)
            .score(founded_doc.score)
            .document(document)
            .highlight(highlight)
            .build()?;

        Ok(result)
    }
}
