use derive_builder::Builder;
use doc_search_core::domain::searcher::models::{FoundedDocument, FoundedDocumentBuilder};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(unused_imports)]
use serde_json::json;

use crate::server::httpserver::api::v1::schema::document::DocumentPartSchema;
use crate::server::ServerError;

#[derive(Builder, Clone, Deserialize, Serialize, ToSchema)]
pub struct FoundedDocumentPartSchema {
    #[schema(example = "29346839246dsf987a1173sfa7sd781h")]
    pub id: String,
    #[schema(example = "test-folder")]
    pub index: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = 0.7523)]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[schema(example = json!(vec!["There is", "some text"]))]
    pub highlight: Vec<String>,
    pub document: DocumentPartSchema,
}

impl TryFrom<FoundedDocumentPartSchema> for FoundedDocument {
    type Error = ServerError;

    fn try_from(schema: FoundedDocumentPartSchema) -> Result<Self, Self::Error> {
        let document = schema.document.try_into()?;
        FoundedDocumentBuilder::default()
            .id(schema.id)
            .index(schema.index)
            .score(schema.score)
            .highlight(schema.highlight)
            .document(document)
            .build()
            .map_err(|err| ServerError::InternalError(err.to_string()))
    }
}

impl TryFrom<FoundedDocument> for FoundedDocumentPartSchema {
    type Error = ServerError;

    fn try_from(founded: FoundedDocument) -> Result<Self, Self::Error> {
        let document = founded.document.try_into()?;
        FoundedDocumentPartSchemaBuilder::default()
            .id(founded.id.to_string())
            .index(founded.index)
            .score(founded.score)
            .highlight(founded.highlight)
            .document(document)
            .build()
            .map_err(|err| ServerError::InternalError(err.to_string()))
    }
}
