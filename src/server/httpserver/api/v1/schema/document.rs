use derive_builder::Builder;
use doc_search_core::domain::searcher::models::{
    DocumentPartEntrails, DocumentPartEntrailsBuilder, Embeddings,
};
use doc_search_core::domain::storage::models::{
    DocumentPart, DocumentPartBuilder, StoredDocumentPartsInfo,
};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(unused_imports)]
use serde_json::json;

use crate::server::ServerError;

#[derive(Builder, Clone, Serialize, ToSchema)]
pub struct StoredDocumentSchema {
    #[schema(example = "dksfsjvJHZVFDskjdbfsdfsdfdsg")]
    large_doc_id: String,
    #[schema(example = "3b4kb534k5bkqjb1kj3b21kj23b")]
    first_part_id: String,
    #[schema(example = 10)]
    doc_parts_amount: u64,
}

impl TryFrom<StoredDocumentPartsInfo> for StoredDocumentSchema {
    type Error = ServerError;

    fn try_from(doc: StoredDocumentPartsInfo) -> Result<Self, Self::Error> {
        StoredDocumentSchemaBuilder::default()
            .large_doc_id(doc.large_doc_id)
            .first_part_id(doc.first_part_id)
            .doc_parts_amount(doc.doc_parts_amount as u64)
            .build()
            .map_err(|err| ServerError::InternalError(err.to_string()))
    }
}

#[derive(Builder, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct DocumentPartSchema {
    #[schema(example = "dksfsjvJHZVFDskjdbfsdfsdfdsg")]
    large_doc_id: String,
    #[schema(example = 0)]
    doc_part_id: u32,
    #[schema(example = "test-document.docx")]
    file_name: String,
    #[schema(example = "./test-document.docx")]
    file_path: String,
    #[schema(example = 1024)]
    file_size: u32,
    #[schema(example = 1750957115)]
    created_at: i64,
    #[schema(example = 1750957115)]
    modified_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "There is some content data")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable)]
    chunked_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable)]
    embeddings: Option<Vec<EmbeddingsSchema>>,
}

impl TryFrom<DocumentPartSchema> for DocumentPart {
    type Error = ServerError;

    fn try_from(schema: DocumentPartSchema) -> Result<Self, Self::Error> {
        DocumentPartBuilder::default()
            .large_doc_id(schema.large_doc_id)
            .doc_part_id(schema.doc_part_id as usize)
            .file_name(schema.file_name)
            .file_path(schema.file_path)
            .file_size(schema.file_size)
            .created_at(schema.created_at)
            .modified_at(schema.modified_at)
            .content(schema.content.unwrap_or_default())
            .build()
            .map_err(|err| ServerError::InternalError(err.to_string()))
    }
}

impl TryFrom<DocumentPart> for DocumentPartSchema {
    type Error = ServerError;

    fn try_from(doc_part: DocumentPart) -> Result<Self, Self::Error> {
        DocumentPartSchemaBuilder::default()
            .large_doc_id(doc_part.large_doc_id)
            .doc_part_id(doc_part.doc_part_id as u32)
            .file_name(doc_part.file_name)
            .file_path(doc_part.file_path)
            .file_size(doc_part.file_size)
            .created_at(doc_part.created_at)
            .modified_at(doc_part.modified_at)
            .content(Some(doc_part.content))
            .chunked_text(None)
            .embeddings(None)
            .build()
            .map_err(|err| ServerError::InternalError(err.to_string()))
    }
}

impl TryFrom<DocumentPartSchema> for DocumentPartEntrails {
    type Error = ServerError;

    fn try_from(schema: DocumentPartSchema) -> Result<Self, Self::Error> {
        let embeddings = schema
            .embeddings
            .map(|it| {
                it.into_iter()
                    .map(|e| e.into())
                    .collect::<Vec<Embeddings>>()
            })
            .unwrap_or_default();

        DocumentPartEntrailsBuilder::default()
            .large_doc_id(schema.large_doc_id)
            .doc_part_id(schema.doc_part_id as usize)
            .file_name(schema.file_name)
            .file_path(schema.file_path)
            .file_size(schema.file_size)
            .created_at(schema.created_at)
            .modified_at(schema.modified_at)
            .content(schema.content)
            .chunked_text(schema.chunked_text)
            .embeddings(Some(embeddings))
            .build()
            .map_err(|err| ServerError::InternalError(err.to_string()))
    }
}

impl TryFrom<DocumentPartEntrails> for DocumentPartSchema {
    type Error = ServerError;

    fn try_from(doc_entrails: DocumentPartEntrails) -> Result<Self, Self::Error> {
        let embeddings = doc_entrails.embeddings.map(|it| {
            it.into_iter()
                .map(EmbeddingsSchema::try_from)
                .filter_map(Result::ok)
                .collect()
        });

        DocumentPartSchemaBuilder::default()
            .large_doc_id(doc_entrails.large_doc_id)
            .doc_part_id(doc_entrails.doc_part_id as u32)
            .file_name(doc_entrails.file_name)
            .file_path(doc_entrails.file_path)
            .file_size(doc_entrails.file_size)
            .created_at(doc_entrails.created_at)
            .modified_at(doc_entrails.modified_at)
            .content(doc_entrails.content)
            .chunked_text(doc_entrails.chunked_text)
            .embeddings(embeddings)
            .build()
            .map_err(|err| ServerError::InternalError(err.to_string()))
    }
}

#[derive(Builder, Clone, Serialize, Deserialize, ToSchema)]
pub struct EmbeddingsSchema {
    #[schema(example = json!(vec![1.238473924, -1.0234324]))]
    knn: Vec<f64>,
}

impl From<EmbeddingsSchema> for Embeddings {
    fn from(schema: EmbeddingsSchema) -> Self {
        Embeddings { knn: schema.knn }
    }
}

impl From<Embeddings> for EmbeddingsSchema {
    fn from(embeddings: Embeddings) -> Self {
        EmbeddingsSchemaBuilder::default()
            .knn(embeddings.knn)
            .build()
            .expect("embeddings schema could not be built")
    }
}
