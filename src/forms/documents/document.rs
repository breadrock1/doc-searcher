use crate::forms::documents::embeddings::EmbeddingsVector;
use crate::forms::documents::metadata::{Artifacts, GroupValue, HighlightEntity, OcrMetadata};
use crate::forms::documents::DocumentsTrait;
use crate::forms::TestExample;

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use datetime::{deserialize_dt, serialize_dt};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Builder, Default, Clone, ToSchema)]
pub struct Document {
    #[schema(example = "test_folder")]
    folder_id: String,
    #[schema(example = "/test_folder")]
    folder_path: String,
    #[schema(example = "The Ocean Carrier has been signed.")]
    content: String,
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    #[serde(alias = "document_md5")]
    document_id: String,
    #[schema(example = "12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")]
    document_ssdeep: String,
    #[schema(example = "test_document.txt")]
    document_name: String,
    #[schema(example = "/test_folder/test_document.txt")]
    document_path: String,
    #[schema(example = 35345)]
    document_size: i32,
    #[schema(example = "document")]
    document_type: String,
    #[schema(example = ".txt")]
    document_extension: String,
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
    pub fn get_folder_path(&self) -> &str {
        self.folder_path.as_str()
    }
    pub fn get_content(&self) -> &str {
        self.content.as_str()
    }
    pub fn get_doc_ssdeep(&self) -> &str {
        self.document_ssdeep.as_str()
    }
    pub fn get_doc_name(&self) -> &str {
        self.document_name.as_str()
    }
    pub fn get_doc_path(&self) -> &str {
        self.document_path.as_str()
    }
    pub fn set_doc_path(&mut self, path: &str) {
        self.document_path = path.to_owned();
    }
    pub fn get_doc_size(&self) -> i32 {
        self.document_size
    }
    pub fn get_doc_type(&self) -> &str {
        self.document_type.as_str()
    }
    pub fn get_doc_ext(&self) -> &str {
        self.document_extension.as_str()
    }
    pub fn get_doc_perm(&self) -> i32 {
        self.document_permissions
    }
    pub fn get_doc_created(&self) -> Option<&DateTime<Utc>> {
        self.document_created.as_ref()
    }
    pub fn get_doc_modified(&self) -> Option<&DateTime<Utc>> {
        self.document_modified.as_ref()
    }
    pub fn get_ocr_quality(&self) -> Option<i32> {
        self.quality_recognition
    }
    pub fn get_ocr_metadata(&self) -> Option<&OcrMetadata> {
        self.ocr_metadata.as_ref()
    }
    pub fn get_embeddings(&self) -> Vec<EmbeddingsVector> {
        match &self.embeddings {
            None => Vec::default(),
            Some(vector) => vector.to_vec(),
        }
    }
    pub fn append_highlight(&mut self, highlight: Option<HighlightEntity>) {
        self.highlight = highlight
    }
    pub fn exclude_tokens(&mut self) {
        self.embeddings = None;
    }
    pub fn set_folder_path(&mut self, folder_path: &str) {
        self.folder_path = folder_path.to_string()
    }
    pub fn set_artifacts(&mut self, artifacts: Artifacts) {
        let mut ocr_metadata = self
            .get_ocr_metadata()
            .cloned()
            .unwrap_or_else(|| {
                OcrMetadata::builder()
                    .job_id(String::default())
                    .pages_count(0)
                    .doc_type(artifacts.get_group_name().to_string())
                    .artifacts(None)
                    .build()
                    .unwrap()
            });

        if ocr_metadata.get_artifacts().is_none() {
            ocr_metadata.set_artifacts(Some(vec![artifacts]))
        }

        self.ocr_metadata = Some(ocr_metadata)
    }
}

impl DocumentsTrait for Document {
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

impl TestExample<Document> for Document {
    fn test_example(_val: Option<&str>) -> Document {
        let created = NaiveDateTime::default()
            .with_year(2024)
            .unwrap()
            .with_month(4)
            .unwrap()
            .with_day(3)
            .unwrap()
            .with_hour(13)
            .unwrap()
            .with_minute(51)
            .unwrap()
            .with_second(32)
            .unwrap()
            .and_utc();

        let modified = NaiveDateTime::default()
            .with_year(2024)
            .unwrap()
            .with_month(4)
            .unwrap()
            .with_day(25)
            .unwrap()
            .with_hour(11)
            .unwrap()
            .with_minute(14)
            .unwrap()
            .with_second(55)
            .unwrap()
            .and_utc();

        let group_values = vec![GroupValue::builder()
            .name("Date of TN".to_string())
            .json_name("date_of_tn".to_string())
            .group_type("string".to_string())
            .value(Some("2023-10-29".to_string()))
            .build()
            .unwrap()];

        let artifacts = vec![Artifacts::builder()
            .group_name("Information of TN".to_string())
            .group_json_name("tn_info".to_string())
            .group_values(Some(group_values))
            .build()
            .unwrap()];

        let ocr_metadata = OcrMetadata::builder()
            .job_id("c643c506-f5c3-4262-991d-bbe847035499".to_string())
            .pages_count(1)
            .doc_type("TN".to_string())
            .artifacts(Some(artifacts))
            .build()
            .unwrap();

        DocumentBuilder::default()
            .folder_id("test_folder".to_string())
            .folder_path("/test_folder".to_string())
            .document_id("98ac9896be35f47fb8442580cd9839b4".to_string())
            .document_ssdeep("12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y".to_string())
            .document_name("test_document.txt".to_string())
            .document_path("/test_folder/test_document.txt".to_string())
            .document_size(35345)
            .document_type("document".to_string())
            .document_extension(".txt".to_string())
            .document_permissions(777)
            .content("The Ocean Carrier has been signed.".to_string())
            .document_created(Some(created))
            .document_modified(Some(modified))
            .quality_recognition(Some(10000))
            .highlight(None)
            .ocr_metadata(Some(ocr_metadata))
            .embeddings(Some(vec![]))
            .build()
            .unwrap()
    }
}
