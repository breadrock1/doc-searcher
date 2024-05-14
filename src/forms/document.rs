use crate::forms::TestExample;
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

use datetime::{deserialize_dt, serialize_dt};
use derive_builder::Builder;
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, Builder, Default, Clone, ToSchema)]
pub struct Document {
    #[schema(example = "test_folder")]
    folder_id: String,
    #[schema(example = "/test_folder")]
    folder_path: String,
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    content_md5: String,
    #[schema(example = "a9850114-5903-465a-bfc5-8d9e28110be8")]
    content_uuid: String,
    #[schema(example = "The Ocean Carrier has been signed.")]
    content: String,
    #[serde(default)]
    content_vector: Vec<f64>,
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    document_md5: String,
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
    highlight: Option<HighlightEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocr_metadata: Option<OcrMetadata>,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::default()
    }

    pub fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }

    pub fn get_folder_path(&self) -> &str {
        self.folder_path.as_str()
    }

    pub fn get_doc_id(&self) -> &str {
        self.content_md5.as_str()
    }

    pub fn get_content_uuid(&self) -> &str {
        self.content_uuid.as_str()
    }

    pub fn get_content(&self) -> &str {
        self.content.as_str()
    }

    pub fn get_convent_vector(&self) -> &Vec<f64> {
        self.content_vector.as_ref()
    }

    pub fn get_doc_md5(&self) -> &str {
        self.document_md5.as_str()
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

    pub fn set_doc_path(&mut self, path: &str) {
        self.document_path = path.to_owned();
    }

    pub fn append_highlight(&mut self, highlight: Option<HighlightEntity>) {
        self.highlight = highlight
    }

    pub fn exclude_tokens(&mut self) {
        self.content_vector = Vec::default();
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

        DocumentBuilder::default()
            .folder_id("test_folder".to_string())
            .folder_path("/test_folder".to_string())
            .document_md5("98ac9896be35f47fb8442580cd9839b4".to_string())
            .document_ssdeep("12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y".to_string())
            .document_name("test_document.txt".to_string())
            .document_path("/test_folder/test_document.txt".to_string())
            .document_size(35345)
            .document_type("document".to_string())
            .document_extension(".txt".to_string())
            .document_permissions(777)
            .content_md5("98ac9896be35f47fb8442580cd9839b4".to_string())
            .content_uuid("a9850114-5903-465a-bfc5-8d9e28110be8".to_string())
            .content("The Ocean Carrier has been signed.".to_string())
            .content_vector(Vec::default())
            .document_created(Some(created))
            .document_modified(Some(modified))
            .quality_recognition(None)
            .highlight(Some(HighlightEntity {
                content: vec!["Ocean Carrier".to_string()],
            }))
            .ocr_metadata(Some(OcrMetadata {
                job_id: "c643c506-f5c3-4262-991d-bbe847035499".to_string(),
                text: "".to_string(),
                pages_count: 1,
                doc_type: "SMGS".to_string(),
                artifacts: None,
            }))
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Deserialize, Serialize, ToSchema)]
pub struct OcrMetadata {
    #[schema(example = "c643c506-f5c3-4262-991d-bbe847035499")]
    pub job_id: String,
    pub text: String,
    #[schema(example = 1)]
    pub pages_count: i32,
    #[schema(example = "Коносамент")]
    pub doc_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<Artifacts>>,
}

#[derive(Builder, Clone, Deserialize, Serialize, ToSchema)]
pub struct Artifacts {
    #[schema(example = "Information of TN")]
    pub group_name: String,
    #[schema(example = "tn_info")]
    pub group_json_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_values: Option<Vec<GroupValue>>,
}

#[derive(Builder, Clone, Deserialize, Serialize, ToSchema)]
pub struct GroupValue {
    #[schema(example = "Date of TN")]
    pub name: String,
    #[schema(example = "date_of_tn")]
    pub json_name: String,
    #[schema(example = "string")]
    #[serde(rename = "type")]
    pub group_type: String,
    #[schema(example = "2023-10-29")]
    #[serde(deserialize_with = "deser_group_value")]
    pub value: Option<String>,
}

fn deser_group_value<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).and_then(|value| Ok(Some(value.replace("-", "   "))))
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct HighlightEntity {
    pub content: Vec<String>,
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    quality_recognition: Option<i32>,
    #[schema(example = 35345)]
    file_size: i32,
    #[schema(example = "Test Folder")]
    location: String,
    #[schema(example = "test_folder")]
    folder_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview_properties: Option<Vec<Artifacts>>,
}

impl DocumentPreview {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }

    pub fn get_location(&self) -> &str {
        self.location.as_str()
    }
}

impl TestExample<DocumentPreview> for DocumentPreview {
    fn test_example(_val: Option<&str>) -> DocumentPreview {
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

        DocumentPreviewBuilder::default()
            .id("98ac9896be35f47fb8442580cd9839b4".to_string())
            .name("test_document.txt".to_string())
            .created_at(Some(created))
            .quality_recognition(Some(10000))
            .file_size(35345)
            .location("test_folder".to_string())
            .preview_properties(
                vec![ArtifactsBuilder::default()
                    .group_name("Information of TN".to_string())
                    .group_json_name("tn_info".to_string())
                    .group_values(
                        vec![GroupValueBuilder::default()
                            .name("Date of TN".to_string())
                            .json_name("date_of_tn".to_string())
                            .group_type("string".to_string())
                            .value(Some("2023-10-29".to_string()))
                            .build()
                            .unwrap()]
                        .into(),
                    )
                    .build()
                    .unwrap()]
                .into(),
            )
            .build()
            .unwrap()
    }
}

impl From<Document> for DocumentPreview {
    fn from(value: Document) -> Self {
        DocumentPreviewBuilder::default()
            .id(value.document_md5)
            .name(value.document_name)
            .created_at(value.document_created)
            .quality_recognition(Some(10000))
            .file_size(value.document_size)
            .location(value.folder_id)
            .preview_properties(Some(Vec::default()))
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct MoveDocumetsForm {
    document_ids: Vec<String>,
    location: String,
    src_folder_id: String,
}

impl MoveDocumetsForm {
    pub fn get_folder_id(&self) -> &str {
        self.location.as_str()
    }

    pub fn get_src_folder_id(&self) -> &str {
        self.src_folder_id.as_str()
    }

    pub fn get_document_ids(&self) -> &[String] {
        self.document_ids.as_slice()
    }
}

impl TestExample<MoveDocumetsForm> for MoveDocumetsForm {
    fn test_example(_value: Option<&str>) -> MoveDocumetsForm {
        MoveDocumetsFormBuilder::default()
            .location("Test Folder".to_string())
            .src_folder_id("unrecognized".to_string())
            .document_ids(vec!["98ac9896be35f47fb8442580cd9839b4".to_string()])
            .build()
            .unwrap()
    }
}

#[derive(Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct AnalyseDocumentsForm {
    pub document_ids: Vec<String>,
}

impl TestExample<AnalyseDocumentsForm> for AnalyseDocumentsForm {
    fn test_example(_value: Option<&str>) -> AnalyseDocumentsForm {
        AnalyseDocumentsForm {
            document_ids: vec!["98ac9896be35f47fb8442580cd9839b4".to_string()],
        }
    }
}
