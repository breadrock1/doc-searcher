use crate::storage::forms::{CreateFolderForm, FolderType};
use crate::storage::DocumentsTrait;

use chrono::{DateTime, Utc};
use datetime::{deserialize_dt, serialize_dt};
use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

pub const DEFAULT_FOLDER_ID: &str = "common-folder";
pub const INFO_FOLDER_ID: &str = "info-folder";

#[derive(Builder, Clone, Default, Deserialize, Serialize, Getters, ToSchema)]
pub struct Folder {
    #[schema(example = "yellow")]
    health: String,
    #[schema(example = "open")]
    status: String,
    #[getset(get = "pub")]
    #[schema(example = "test-folder", rename = "id")]
    #[serde(rename(serialize = "id"))]
    index: String,
    #[schema(example = "60qbF_yuTa2TXYd7soYb1A")]
    uuid: String,
    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pri: Option<String>,
    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    rep: Option<String>,
    #[schema(example = "100")]
    #[serde(alias = "docs.count", skip_serializing_if = "Option::is_none")]
    docs_count: Option<String>,
    #[schema(example = "50")]
    #[serde(alias = "docs.deleted", skip_serializing_if = "Option::is_none")]
    docs_deleted: Option<String>,
    #[schema(example = "890.3kb")]
    #[serde(alias = "store.size", skip_serializing_if = "Option::is_none")]
    store_size: Option<String>,
    #[schema(example = "890.3kb")]
    #[serde(alias = "pri.store.size", skip_serializing_if = "Option::is_none")]
    pri_store_size: Option<String>,
    #[schema(example = "Test Folder Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Folder {
    pub fn builder() -> FolderBuilder {
        FolderBuilder::default()
    }

    pub fn docs_count(&self) -> Option<&String> {
        self.docs_count.as_ref()
    }

    pub fn set_docs_count(&mut self, docs_count: &str) {
        self.docs_count = Some(docs_count.to_string());
    }

    pub fn docs_deleted(&self) -> Option<&String> {
        self.docs_deleted.as_ref()
    }

    pub fn get_pri_store_size(&self) -> Option<&String> {
        self.pri_store_size.as_ref()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, Getters, CopyGetters)]
pub struct InfoFolder {
    #[getset(get = "pub")]
    index: String,
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    location: String,
    #[getset(get = "pub")]
    user_id: String,
    #[getset(get = "pub")]
    folder_type: FolderType,
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
            location: value.location().to_owned(),
            user_id: value.user_id().to_owned(),
            folder_type: value.folder_type().to_owned(),
            is_system: value.is_system(),
        }
    }
}

#[derive(Builder, Clone, Default, Deserialize, Getters, CopyGetters, Serialize, ToSchema)]
pub struct Document {
    #[getset(get = "pub")]
    #[schema(example = "test-folder")]
    folder_id: String,
    #[getset(get = "pub")]
    #[schema(example = "/test-folder")]
    folder_path: String,
    #[getset(get = "pub")]
    #[schema(example = "The Ocean Carrier has been signed.")]
    content: String,
    #[getset(get = "pub")]
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    #[serde(alias = "document_md5")]
    document_id: String,
    #[getset(get = "pub")]
    #[schema(example = "12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")]
    document_ssdeep: String,
    #[getset(get = "pub")]
    #[schema(example = "test_document.txt")]
    document_name: String,
    #[getset(get = "pub")]
    #[schema(example = "/test-folder/test_document.txt")]
    document_path: String,
    #[getset(get_copy = "pub")]
    #[schema(example = 35345)]
    document_size: i32,
    #[getset(get = "pub")]
    #[schema(example = "document")]
    document_type: String,
    #[getset(get = "pub")]
    #[schema(example = ".txt")]
    document_extension: String,
    #[getset(get_copy = "pub")]
    #[schema(example = 777)]
    document_permissions: i32,
    #[getset(get_copy = "pub")]
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-03T13:51:32Z")]
    document_created: Option<DateTime<Utc>>,
    #[getset(get_copy = "pub")]
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-25T11:14:55Z")]
    document_modified: Option<DateTime<Utc>>,
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    quality_recognition: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocr_metadata: Option<OcrMetadata>,
    highlight: Option<HighlightEntity>,
    #[serde(skip_serializing_if = "Option::is_none", default = "Option::default")]
    embeddings: Option<Vec<EmbeddingsVector>>,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::default()
    }

    pub fn ocr_metadata(&self) -> Option<&OcrMetadata> {
        self.ocr_metadata.as_ref()
    }

    pub fn get_embeddings(&self) -> Vec<EmbeddingsVector> {
        self.embeddings.clone().unwrap_or_default()
    }

    pub fn append_highlight(&mut self, highlight: Option<HighlightEntity>) {
        self.highlight = highlight
    }

    pub fn exclude_tokens(&mut self) {
        self.embeddings = None;
    }

    pub fn set_artifacts(&mut self, doc_artifacts: &DocsArtifacts) {
        let ocr_meta = self.ocr_metadata().cloned();
        let mut ocr_metadata = ocr_meta.unwrap_or_else(|| {
            OcrMetadata::builder()
                .job_id(String::default())
                .pages_count(0)
                .doc_type(String::default())
                .artifacts(None)
                .build()
                .unwrap()
        });

        let doc_type = doc_artifacts.name();
        ocr_metadata.set_doc_type(doc_type);

        let artifacts = doc_artifacts.artifacts().to_vec();
        ocr_metadata.set_artifacts(Some(artifacts));

        self.ocr_metadata = Some(ocr_metadata.to_owned())
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

impl From<&Document> for Document {
    fn from(value: &Document) -> Self {
        Document::builder()
            .folder_id(value.folder_id.to_owned())
            .folder_path(value.folder_path.to_owned())
            .document_id(value.document_id.to_owned())
            .document_ssdeep(value.document_ssdeep.to_owned())
            .document_name(value.document_name.to_owned())
            .document_path(value.document_path.to_owned())
            .document_size(value.document_size.to_owned())
            .document_type(value.document_type.to_owned())
            .document_extension(value.document_extension.to_owned())
            .document_permissions(value.document_permissions.to_owned())
            .content(value.content.to_owned())
            .document_created(value.document_created.to_owned())
            .document_modified(value.document_modified.to_owned())
            .quality_recognition(value.quality_recognition.to_owned())
            .highlight(None)
            .ocr_metadata(value.ocr_metadata.to_owned())
            .embeddings(None)
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Deserialize, Serialize, Getters, CopyGetters, ToSchema)]
pub struct OcrMetadata {
    #[getset(get = "pub")]
    #[schema(example = "c643c506-f5c3-4262-991d-bbe847035499")]
    job_id: String,
    #[getset(get_copy = "pub")]
    #[schema(example = 1)]
    pages_count: i32,
    #[getset(get = "pub")]
    #[schema(example = "Коносамент")]
    doc_type: String,
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    artifacts: Option<Vec<Artifacts>>,
}

impl OcrMetadata {
    pub fn builder() -> OcrMetadataBuilder {
        OcrMetadataBuilder::default()
    }

    pub fn get_artifacts(&self) -> Option<&Vec<Artifacts>> {
        self.artifacts.as_ref()
    }

    pub fn set_artifacts(&mut self, artifacts: Option<Vec<Artifacts>>) {
        self.artifacts = artifacts
    }

    pub fn set_doc_type(&mut self, doc_type: &str) {
        self.doc_type = doc_type.to_string();
    }
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, Getters, ToSchema)]
pub struct DocsArtifacts {
    #[getset(get = "pub")]
    name: String,
    json_name: String,
    sample_file_name: String,
    #[getset(get = "pub")]
    artifacts: Vec<Artifacts>,
}

impl Default for DocsArtifacts {
    fn default() -> Self {
        DocsArtifacts {
            name: "unknown".to_string(),
            json_name: "unknown".to_string(),
            sample_file_name: "PLACE_SAMPLE_FILE_NAME".to_string(),
            artifacts: Vec::default(),
        }
    }
}

#[derive(Builder, Clone, Deserialize, Serialize, ToSchema)]
pub struct Artifacts {
    #[schema(example = "Information of TN")]
    group_name: String,
    #[schema(example = "tn_info")]
    group_json_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_values: Option<Vec<GroupValue>>,
}

impl Default for Artifacts {
    fn default() -> Self {
        Artifacts {
            group_name: "unknown".to_string(),
            group_json_name: "unknown".to_string(),
            group_values: None,
        }
    }
}

impl Artifacts {
    pub fn builder() -> ArtifactsBuilder {
        ArtifactsBuilder::default()
    }

    pub fn get_group_name(&self) -> &str {
        self.group_name.as_str()
    }
}

#[derive(Builder, Clone, Deserialize, Serialize, ToSchema)]
pub struct GroupValue {
    #[schema(example = "Date of TN")]
    name: String,
    #[schema(example = "date_of_tn")]
    json_name: String,
    #[schema(example = "string")]
    #[serde(rename = "type")]
    group_type: String,
    #[schema(example = "2023-10-29")]
    #[serde(default, deserialize_with = "deser_group_value")]
    value: Option<String>,
}

impl GroupValue {
    pub fn builder() -> GroupValueBuilder {
        GroupValueBuilder::default()
    }
}

fn deser_group_value<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let str_value = match value {
        Value::Null => String::default(),
        Value::Bool(val) => val.to_string(),
        Value::Number(val) => val.to_string(),
        Value::String(val) => val,
        _ => value.to_string(),
    };

    Ok(Some(str_value.replace('-', "   ")))
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct HighlightEntity {
    pub content: Vec<String>,
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, Getters, CopyGetters, ToSchema)]
pub struct DocumentPreview {
    #[getset(get = "pub")]
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    id: String,
    #[getset(get = "pub")]
    #[schema(example = "test_document.txt")]
    name: String,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-03T13:51:32Z")]
    created_at: Option<DateTime<Utc>>,
    #[getset(get_copy = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    quality_recognition: Option<i32>,
    #[getset(get_copy = "pub")]
    #[schema(example = 35345)]
    file_size: i32,
    #[getset(get = "pub")]
    #[schema(example = "Test Folder")]
    location: String,
    #[getset(get = "pub")]
    #[schema(example = "test-folder")]
    folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview_properties: Option<Vec<Artifacts>>,
}

#[allow(dead_code)]
impl DocumentPreview {
    pub fn builder() -> DocumentPreviewBuilder {
        DocumentPreviewBuilder::default()
    }

    pub fn get_artifacts(&self) -> Option<&Vec<Artifacts>> {
        self.preview_properties.as_ref()
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
        let ocr_metadata = value.ocr_metadata().cloned();
        let artifacts = match ocr_metadata {
            Some(metadata) => metadata.artifacts().clone(),
            None => None,
        };

        DocumentPreview::builder()
            .id(value.document_id().to_owned())
            .folder_id(value.folder_id().to_owned())
            .name(value.document_name().to_owned())
            .location(value.folder_id().to_owned())
            .created_at(value.document_created())
            .quality_recognition(value.quality_recognition())
            .file_size(value.document_size())
            .preview_properties(artifacts)
            .build()
            .unwrap()
    }
}

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
}

impl From<&Document> for DocumentVectors {
    fn from(value: &Document) -> Self {
        let mut test = DocumentVectors::builder()
            .folder_id(value.folder_id().to_owned())
            .document_id(value.document_id().to_owned())
            .document_name(value.document_name().to_owned())
            .document_modified(value.document_modified())
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
