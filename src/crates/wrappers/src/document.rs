use crate::TestExample;
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

use datetime::{deserialize_dt, serialize_dt};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, Builder, Default, Clone, ToSchema)]
pub struct Document {
    #[schema(example = "test_folder")]
    pub folder_id: String,
    #[schema(example = "/test_folder")]
    pub folder_path: String,
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    pub document_md5: String,
    #[schema(example = "12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")]
    pub document_ssdeep: String,
    #[schema(example = "test_document.txt")]
    pub document_name: String,
    #[schema(example = "/test_folder/test_document.txt")]
    pub document_path: String,
    #[schema(example = 35345)]
    pub document_size: i32,
    #[schema(example = "document")]
    pub document_type: String,
    #[schema(example = ".txt")]
    pub document_extension: String,
    #[schema(example = 777)]
    pub document_permissions: i32,
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    pub content_md5: String,
    #[schema(example = "a9850114-5903-465a-bfc5-8d9e28110be8")]
    pub content_uuid: String,
    #[schema(example = "The Ocean Carrier has been signed.")]
    pub content: String,
    #[serde(default)]
    pub content_vector: Vec<f64>,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-03T13:51:32Z")]
    pub document_created: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-25T11:14:55Z")]
    pub document_modified: Option<DateTime<Utc>>,
    pub highlight: Option<HighlightEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_metadata: Option<OcrMetadata>,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::default()
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
    pub artifacts: Option<Artifacts>,
}

#[derive(Builder, Clone, Deserialize, Serialize, ToSchema)]
pub struct Artifacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_invoice_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_invoice_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_date_arrival: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_aate_departure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_redirection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_redirection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_issue_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_issue_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_places_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_receipt_act_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_receipt_act_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ktk_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_full_name: Option<String>,
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct HighlightEntity {
    pub content: Vec<String>,
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct DocumentPreview {
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    pub id: String,
    #[schema(example = "test_document.txt")]
    pub name: String,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    #[schema(example = "2024-04-03T13:51:32Z")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_recognization: Option<i32>,
    #[schema(example = 35345)]
    pub file_size: i32,
    #[schema(example = "test_folder")]
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_properties: Option<Vec<PreviewProperties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Properties>>,
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
            .quality_recognization(Some(10000))
            .file_size(35345)
            .location("test_folder".to_string())
            .preview_properties(vec![PreviewPropertiesBuilder::default()
                .name("transfer_company".to_string())
                .key("Перевозчик".to_string())
                .value("ООО Мостранс".to_string())
                .build()
                .unwrap()].into())
            .properties(None)
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
            .quality_recognization(Some(10000))
            .file_size(value.document_size)
            .location(value.folder_id)
            .preview_properties(Some(Vec::default()))
            .build()
            .unwrap()
    }
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct PreviewProperties {
    #[schema(example = "transfer_company")]
    pub key: String,
    #[schema(example = "Перевозчик")]
    pub name: String,
    #[schema(example = "ООО Мостранс")]
    pub value: String,
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct Properties {
    #[schema(example = "Приём груза")]
    pub group_name: String,
    pub group_values: Vec<PreviewProperties>,
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct MoveDocumetsForm {
    document_ids: Vec<String>,
    folder_id: String,
}

impl MoveDocumetsForm {
    pub fn get_folder_id(&self) -> &str {
        self.folder_id.as_str()
    }
    
    pub fn get_document_ids(&self) -> &[String] {
        self.document_ids.as_slice()
    }
}

impl TestExample<MoveDocumetsForm> for MoveDocumetsForm {
    fn test_example(_value: Option<&str>) -> MoveDocumetsForm {
        MoveDocumetsFormBuilder::default()
            .folder_id("test_folder".to_string())
            .document_ids(vec!["98ac9896be35f47fb8442580cd9839b4".to_string()])
            .build()
            .unwrap()
    }
}
