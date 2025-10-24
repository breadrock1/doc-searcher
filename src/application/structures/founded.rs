use derive_builder::Builder;
use gset::Getset;
use serde_derive::Serialize;

use crate::application::structures::DocumentPart;

#[derive(Builder, Getset, Clone, Serialize)]
pub struct FoundedDocument {
    #[getset(get, vis = "pub")]
    id: String,
    #[getset(get, vis = "pub")]
    folder_id: String,
    #[getset(get, vis = "pub")]
    document: DocumentPart,
    #[getset(get_copy, vis = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<f64>,
    #[getset(get, vis = "pub")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    highlight: Vec<String>,
}

impl TryFrom<crate::domain::FoundedDocument> for FoundedDocument {
    type Error = anyhow::Error;

    fn try_from(founded_doc: crate::domain::FoundedDocument) -> Result<Self, Self::Error> {
        let highlight = founded_doc.highlight;
        let document = DocumentPart::try_from(founded_doc.document)?;
        let result = FoundedDocumentBuilder::default()
            .id(founded_doc.id)
            .folder_id(founded_doc.folder_id)
            .score(founded_doc.score)
            .document(document)
            .highlight(highlight)
            .build()?;

        Ok(result)
    }
}
