use crate::storage::forms::CreateFolderForm;

use chrono::{DateTime, Utc};
use datetime::{deserialize_dt, serialize_dt};
use derive_builder::Builder;
use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use std::str::FromStr;
use utoipa::ToSchema;

pub const DEFAULT_FOLDER_ID: &str = "common-folder";
pub const INFO_FOLDER_ID: &str = "info-folder";

pub trait DocumentsTrait {
    fn get_folder_id(&self) -> &str;
    fn get_doc_id(&self) -> &str;
}

#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FolderType {
    #[default]
    Document,
    Vectors,
    InfoFolder,
}

#[derive(
    Builder, Clone, Default, Deserialize, Serialize, Getters, CopyGetters, Setters, ToSchema,
)]
#[getset(get = "pub")]
pub struct Folder {
    #[schema(example = "yellow")]
    health: String,

    #[schema(example = "open")]
    status: String,

    #[serde(rename(serialize = "id"))]
    #[schema(example = "test-folder", rename = "id")]
    index: String,

    #[schema(example = "60qbF_yuTa2TXYd7soYb1A")]
    uuid: String,

    #[schema(example = "Test Folder Name")]
    #[getset(set = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[schema(example = 1)]
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "from_str_to_i64")]
    pri: Option<i64>,

    #[schema(example = 1)]
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "from_str_to_i64")]
    rep: Option<i64>,

    #[schema(example = 100)]
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[serde(alias = "docs.count", skip_serializing_if = "Option::is_none", deserialize_with = "from_str_to_i64")]
    docs_count: Option<i64>,

    #[schema(example = 50)]
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[serde(alias = "docs.deleted", skip_serializing_if = "Option::is_none", deserialize_with = "from_str_to_i64")]
    docs_deleted: Option<i64>,

    #[schema(example = "890.3kb")]
    #[serde(alias = "store.size", skip_serializing_if = "Option::is_none")]
    store_size: Option<String>,

    #[schema(example = "890.3kb")]
    #[serde(alias = "pri.store.size", skip_serializing_if = "Option::is_none")]
    pri_store_size: Option<String>,
}

fn from_str_to_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(i64::from_str(&s).map_err(D::Error::custom).ok())
}

impl Folder {
    pub fn builder() -> FolderBuilder {
        FolderBuilder::default()
    }

    pub fn get_pri_store_size(&self) -> Option<&String> {
        self.pri_store_size.as_ref()
    }
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, Getters, CopyGetters)]
#[getset(get = "pub")]
pub struct InfoFolder {
    index: String,
    name: String,
    location: String,
    user_id: String,
    folder_type: FolderType,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    is_system: bool,
}

#[allow(dead_code)]
impl InfoFolder {
    pub fn builder() -> InfoFolderBuilder {
        InfoFolderBuilder::default()
    }
}

impl DocumentsTrait for InfoFolder {
    fn get_folder_id(&self) -> &str {
        "info-folder"
    }

    fn get_doc_id(&self) -> &str {
        self.index.as_str()
    }
}

impl From<&CreateFolderForm> for InfoFolder {
    fn from(value: &CreateFolderForm) -> Self {
        InfoFolder {
            index: value.folder_id().to_owned(),
            name: value.folder_name().to_owned(),
            folder_type: value.folder_type().to_owned(),
            location: value.location().to_owned(),
            user_id: value.user_id().to_owned(),
            is_system: value.is_system(),
        }
    }
}

#[derive(
    Builder, Clone, Default, Deserialize, Serialize, Getters, CopyGetters, Setters, ToSchema,
)]
#[getset(get = "pub")]
pub struct Document {
    #[schema(example = "test-folder")]
    folder_id: String,

    #[schema(example = "/test-folder")]
    folder_path: String,

    #[schema(example = "The Ocean Carrier has been signed.")]
    content: String,

    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    document_id: String,

    #[schema(example = "12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")]
    document_ssdeep: String,

    #[schema(example = "test_document.txt")]
    document_name: String,

    #[schema(example = "/test-folder/test_document.txt")]
    document_path: String,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[schema(example = 35345)]
    document_size: i32,

    #[schema(example = "document")]
    document_type: String,

    #[schema(example = ".txt")]
    document_extension: String,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[schema(example = 777)]
    document_permissions: i32,

    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-03T13:51:32Z")]
    document_created: Option<DateTime<Utc>>,

    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-25T11:14:55Z")]
    document_modified: Option<DateTime<Utc>>,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    quality_recognition: Option<i32>,

    #[getset(set = "pub")]
    highlight: Option<HighlightEntity>,

    #[serde(skip_serializing_if = "Option::is_none", default = "Option::default")]
    embeddings: Option<Vec<EmbeddingsVector>>,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::default()
    }

    pub fn exclude_tokens(&mut self) {
        self.embeddings = None;
    }
}

impl DocumentsTrait for Document {
    fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }

    fn get_doc_id(&self) -> &str {
        self.document_id.as_str()
    }
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct HighlightEntity {
    content: Vec<String>,
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, Getters, CopyGetters, ToSchema)]
#[getset(get = "pub")]
pub struct DocumentPreview {
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    id: String,

    #[schema(example = "test_document.txt")]
    name: String,

    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-03T13:51:32Z")]
    created_at: Option<DateTime<Utc>>,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    quality_recognition: Option<i32>,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[schema(example = 35345)]
    file_size: i32,

    #[schema(example = "Test Folder")]
    location: String,

    #[schema(example = "test-folder")]
    folder_id: String,
}

#[allow(dead_code)]
impl DocumentPreview {
    pub fn builder() -> DocumentPreviewBuilder {
        DocumentPreviewBuilder::default()
    }

    pub fn created_date(&self) -> Option<&DateTime<Utc>> {
        self.created_at.as_ref()
    }
}

impl DocumentsTrait for DocumentPreview {
    fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }

    fn get_doc_id(&self) -> &str {
        self.id.as_str()
    }
}

impl From<&Document> for DocumentPreview {
    fn from(value: &Document) -> Self {
        DocumentPreview::builder()
            .id(value.document_id().to_owned())
            .folder_id(value.folder_id().to_owned())
            .name(value.document_name().to_owned())
            .location(value.folder_id().to_owned())
            .created_at(value.document_created().to_owned())
            .quality_recognition(value.quality_recognition())
            .file_size(value.document_size())
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, Getters, Setters, ToSchema)]
#[getset(get = "pub")]
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

    #[getset(set = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    match_score: Option<f64>,

    #[getset(set = "pub")]
    embeddings: Vec<EmbeddingsVector>,
}

#[derive(Clone, Default, Deserialize, Serialize, Getters, ToSchema)]
#[getset(get = "pub")]
pub struct EmbeddingsVector {
    #[schema(example = "18070394574500154a8ab333a3362aa8")]
    chunk_id: String,

    #[schema(example = "The Ocean Carrier has been signed.")]
    text_chunk: String,

    #[schema(example = "[0.0345456, -0.4353242]")]
    vector: Vec<f64>,
}

impl DocumentVectors {
    pub fn builder() -> DocumentVectorsBuilder {
        DocumentVectorsBuilder::default()
    }

    pub fn exclude_embeddings(&mut self) {
        self.embeddings = Vec::default();
    }

    pub fn exclude_tokens(&mut self) {
        for vector in &mut self.embeddings {
            vector.vector = Vec::default();
        }
    }

    pub fn append_embeddings(&mut self, embeds: EmbeddingsVector) {
        self.embeddings.push(embeds);
    }
}

impl DocumentsTrait for DocumentVectors {
    fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }

    fn get_doc_id(&self) -> &str {
        self.document_id.as_str()
    }
}

impl From<&Document> for DocumentVectors {
    fn from(value: &Document) -> Self {
        let mut doc_vector = DocumentVectors::builder()
            .folder_id(value.folder_id().to_owned())
            .document_id(value.document_id().to_owned())
            .document_name(value.document_name().to_owned())
            .document_modified(value.document_modified().to_owned())
            .embeddings(Vec::default())
            .match_score(None)
            .build()
            .unwrap();

        let embeds = value.embeddings().to_owned().unwrap_or_default();
        doc_vector.set_embeddings(embeds);
        doc_vector
    }
}

impl From<&DocumentVectors> for Vec<DocumentVectors> {
    fn from(value: &DocumentVectors) -> Self {
        let embeds = value.embeddings.to_vec();

        let mut base_doc_vecs = value.clone();
        base_doc_vecs.exclude_embeddings();

        embeds
            .into_iter()
            .map(|tokens| {
                let mut doc_tokens = base_doc_vecs.clone();
                doc_tokens.append_embeddings(tokens);
                doc_tokens
            })
            .collect()
    }
}
