use crate::errors::WebError;
use crate::forms::documents::document::Document;
use crate::forms::documents::vector::DocumentVectors;
use crate::forms::documents::preview::DocumentPreview;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
pub enum DocumentType {
    #[default]
    #[serde(rename(deserialize = "document", serialize = "document",))]
    Document,
    #[serde(rename(deserialize = "preview", serialize = "preview",))]
    Preview,
    #[serde(rename(deserialize = "vectors", serialize = "vectors",))]
    Vectors,
    #[serde(rename(deserialize = "grouped-vectors", serialize = "grouped-vectors",))]
    GroupedVectors,
}

impl DocumentType {
    pub fn to_value(&self, document: &Document) -> Result<Value, WebError> {
        match self {
            DocumentType::Preview => serde_json::to_value(DocumentPreview::from(document)),
            DocumentType::Vectors => serde_json::to_value(DocumentVectors::from(document)),
            _ => serde_json::to_value(document)
        }
        .map_err(WebError::from)
    }
    pub fn is_vector_type(&self) -> bool {
        matches!(self, DocumentType::Vectors)
    }
}

#[derive(Deserialize, Default, IntoParams, ToSchema)]
pub struct DocTypeQuery {
    document_type: Option<DocumentType>,
}

impl DocTypeQuery {
    pub fn get_type(&self) -> DocumentType {
        self.document_type.clone().unwrap_or(DocumentType::Document)
    }
}
