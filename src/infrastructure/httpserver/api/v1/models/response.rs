use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(unused_imports)]
use serde_json::json;

use crate::application::structures::{
    DocumentPart, DocumentPartBuilder, Embeddings, FoundedDocument, FoundedDocumentBuilder, Index,
    IndexBuilder, Paginated, StoredDocumentPart,
};

#[derive(Builder, Serialize, Deserialize, ToSchema)]
pub struct IndexSchema {
    #[schema(example = "test-folder")]
    id: String,
    #[schema(example = "Test Folder")]
    name: String,
    #[schema(example = "./")]
    path: String,
}

impl From<IndexSchema> for Index {
    fn from(form: IndexSchema) -> Index {
        IndexBuilder::default()
            .id(form.id)
            .name(form.name)
            .path(form.path)
            .build()
            .unwrap()
    }
}

impl From<Index> for IndexSchema {
    fn from(index: Index) -> Self {
        IndexSchemaBuilder::default()
            .id(index.id().to_string())
            .name(index.name().to_string())
            .path(index.path().to_string())
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Serialize, Deserialize, ToSchema)]
pub struct DocumentSchema {
    #[schema(example = "test-document.docx")]
    file_name: String,
    #[schema(example = "./test-document.docx")]
    file_path: String,
    #[schema(example = 1024)]
    file_size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "There is some content data")]
    content: Option<String>,
    #[schema(example = 1750957115)]
    created_at: i64,
    #[schema(example = 1750957115)]
    modified_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable)]
    chunked_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable)]
    embeddings: Option<Vec<EmbeddingsSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable)]
    doc_part_id: Option<usize>,
}

impl From<DocumentSchema> for DocumentPart {
    fn from(form: DocumentSchema) -> Self {
        let embeddings = form
            .embeddings
            .map(|it| {
                it.into_iter()
                    .map(|e| e.into())
                    .collect::<Vec<Embeddings>>()
            })
            .unwrap_or_default();

        let doc_part = form.doc_part_id.unwrap_or(0);
        DocumentPartBuilder::default()
            .file_name(form.file_name)
            .file_path(form.file_path)
            .file_size(form.file_size)
            .content(form.content)
            .created_at(form.created_at)
            .modified_at(form.modified_at)
            .chunked_text(form.chunked_text)
            .embeddings(Some(embeddings))
            .doc_part_id(doc_part)
            .build()
            .unwrap()
    }
}

impl From<DocumentPart> for DocumentSchema {
    fn from(doc: DocumentPart) -> Self {
        let embeddings = doc
            .embeddings()
            .to_owned()
            .map(|it| {
                it.into_iter()
                    .map(|e| e.into())
                    .collect::<Vec<EmbeddingsSchema>>()
            })
            .unwrap_or_default();

        DocumentSchemaBuilder::default()
            .file_name(doc.file_name().to_owned())
            .file_path(doc.file_path().to_owned())
            .file_size(doc.file_size())
            .content(doc.content().to_owned())
            .created_at(doc.created_at())
            .modified_at(doc.modified_at())
            .chunked_text(doc.chunked_text().to_owned())
            .embeddings(Some(embeddings))
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Serialize, Deserialize, ToSchema)]
pub struct EmbeddingsSchema {
    #[schema(example = json!(vec![1.238473924, -1.0234324]))]
    knn: Vec<f64>,
}

impl From<EmbeddingsSchema> for Embeddings {
    fn from(schema: EmbeddingsSchema) -> Self {
        Embeddings::new(schema.knn)
    }
}

impl From<Embeddings> for EmbeddingsSchema {
    fn from(embeddings: Embeddings) -> Self {
        EmbeddingsSchemaBuilder::default()
            .knn(embeddings.knn)
            .build()
            .unwrap()
    }
}

#[derive(Builder, Serialize, Deserialize, ToSchema)]
pub struct PaginatedSchema<D>
where
    D: serde::Serialize + Clone,
{
    #[schema(example = json!(vec![Document::example(None)]))]
    founded: D,
    #[schema(example = "dksfsjvJHZVFDskjdbfsdfsdfdsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}

impl<D> TryFrom<Paginated<D>> for PaginatedSchema<D>
where
    D: serde::Serialize + Clone,
{
    type Error = PaginatedSchemaBuilderError;

    fn try_from(paginated: Paginated<D>) -> Result<Self, Self::Error> {
        PaginatedSchemaBuilder::default()
            .founded(paginated.founded().to_owned())
            .scroll_id(paginated.scroll_id().to_owned())
            .build()
    }
}

#[derive(Builder, Clone, Serialize, ToSchema)]
pub struct FoundedDocumentSchema {
    #[schema(example = "29346839246dsf987a1173sfa7sd781h")]
    id: String,
    #[schema(example = "test-folder")]
    folder_id: String,
    document: DocumentSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = 0.7523)]
    score: Option<f64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[schema(example = json!(vec!["There is", "some text"]))]
    highlight: Vec<String>,
}

impl From<FoundedDocumentSchema> for FoundedDocument {
    fn from(schema: FoundedDocumentSchema) -> Self {
        FoundedDocumentBuilder::default()
            .id(schema.id)
            .folder_id(schema.folder_id)
            .document(schema.document.into())
            .score(schema.score)
            .highlight(schema.highlight)
            .build()
            .unwrap()
    }
}

impl From<FoundedDocument> for FoundedDocumentSchema {
    fn from(founded: FoundedDocument) -> Self {
        FoundedDocumentSchemaBuilder::default()
            .id(founded.id().to_string())
            .folder_id(founded.folder_id().to_string())
            .document(founded.document().to_owned().into())
            .score(founded.score().to_owned())
            .highlight(founded.highlight().to_owned())
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Serialize, ToSchema)]
pub struct StoredDocumentSchema {
    #[schema(example = "dksfsjvJHZVFDskjdbfsdfsdfdsg")]
    id: String,
    #[schema(example = "./test-folder/test-document.docx")]
    file_path: String,
}

impl From<StoredDocumentPart> for StoredDocumentSchema {
    fn from(doc: StoredDocumentPart) -> Self {
        StoredDocumentSchemaBuilder::default()
            .id(doc.id)
            .file_path(doc.file_path)
            .build()
            .unwrap()
    }
}
