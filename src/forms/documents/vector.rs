use crate::forms::documents::document::Document;
use crate::forms::documents::DocumentsTrait;
use crate::forms::TestExample;

use datetime::{deserialize_dt, serialize_dt};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Builder, Default, Clone, ToSchema)]
pub struct DocumentVectors {
    #[schema(example = "test-llama-folder")]
    folder_id: String,
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    document_id: String,
    #[schema(example = "test-document.docx")]
    document_name: String,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-25T11:14:55Z")]
    document_modified: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_score: Option<f64>,
    embeddings: Vec<EmbeddingsVector>,
}

impl DocumentVectors {
    pub fn builder() -> DocumentVectorsBuilder {
        DocumentVectorsBuilder::default()
    }
    pub fn set_match_score(&mut self, score: f64) {
        self.match_score = Some(score)
    }
    pub fn exclude_embeddings(&mut self) {
        self.embeddings = Vec::default();
    }
    pub fn exclude_tokens(&mut self) {
        for vector in &mut self.embeddings {
            vector.vector = Vec::default();
        }
    }
    pub fn get_embeddings(&self) -> &Vec<EmbeddingsVector> {
        &self.embeddings
    }
    pub fn append_embeddings(&mut self, embeds: EmbeddingsVector) {
        self.embeddings.push(embeds);
    }
    pub fn set_embeddings(&mut self, embeds: Vec<EmbeddingsVector>) {
        self.embeddings = embeds;
    }
}

impl DocumentsTrait for DocumentVectors {
    fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }
    fn get_doc_id(&self) -> &str {
        self.document_id.as_str()
    }
    fn set_folder_id(&mut self, folder_id: &str) {
        self.folder_id = folder_id.to_string()
    }
}

impl From<&Document> for DocumentVectors {
    fn from(value: &Document) -> Self {
        let mut test = DocumentVectors::builder()
            .folder_id(value.get_folder_id().to_string())
            .document_id(value.get_doc_id().to_string())
            .document_name(value.get_doc_name().to_string())
            .document_modified(value.get_doc_modified().cloned())
            .embeddings(Vec::default())
            .match_score(None)
            .build()
            .unwrap();

        let test_embeds = value.get_embeddings();
        test.set_embeddings(test_embeds.clone());
        test
    }
}

impl From<&DocumentVectors> for Vec<DocumentVectors> {
    fn from(value: &DocumentVectors) -> Self {
        let embeds = value.embeddings.to_vec();

        let mut base_doc_vecs = value.clone();
        base_doc_vecs.exclude_embeddings();

        embeds
            .into_iter()
            .map(|vecs| {
                let mut doc_vecs = base_doc_vecs.clone();
                doc_vecs.append_embeddings(vecs);
                doc_vecs
            })
            .collect()
    }
}

impl TestExample<DocumentVectors> for DocumentVectors {
    fn test_example(_value: Option<&str>) -> DocumentVectors {
        let local_now = datetime::get_local_now();
        let datetime_now = DateTime::<Utc>::from(local_now);
        DocumentVectors::builder()
            .folder_id("test-folder".to_string())
            .document_id("98ac9896be35f47fb8442580cd9839b4".to_string())
            .document_name("test-document.docx".to_string())
            .document_modified(Some(datetime_now))
            .embeddings(vec![EmbeddingsVector::default()])
            .match_score(None)
            .build()
            .unwrap()
    }
}

#[derive(Deserialize, Serialize, Default, Clone, ToSchema)]
pub struct EmbeddingsVector {
    #[schema(example = "18070394574500154a8ab333a3362aa8")]
    chunk_id: String,
    #[schema(example = "The Ocean Carrier has been signed.")]
    text_chunk: String,
    #[schema(example = "[0.0345456, -0.4353242]")]
    vector: Vec<f64>,
}

impl EmbeddingsVector {
    pub fn get_id(&self) -> &str {
        self.chunk_id.as_str()
    }
}
