use chrono::{DateTime, Utc};
use datetime::{deserialize_dt, serialize_dt};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, Clone, Default, IntoParams, ToSchema)]
pub struct HighlightEntity {
    pub content: Vec<String>,
}

#[derive(Deserialize, Serialize, Builder, Default, Clone, ToSchema)]
pub struct Document {
    pub bucket_uuid: String,
    pub bucket_path: String,
    pub document_md5: String,
    pub document_ssdeep: String,
    pub document_name: String,
    pub document_path: String,
    pub document_size: i32,
    pub document_type: String,
    pub document_extension: String,
    pub document_permissions: i32,
    pub content_md5: String,
    pub content_uuid: String,
    pub content: String,
    #[serde(default)]
    pub content_vector: Vec<f64>,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_created: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_dt",
        deserialize_with = "deserialize_dt",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_modified: Option<DateTime<Utc>>,
    pub highlight: Option<HighlightEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_metadata: Option<OcrMetadata>,
}

impl Document {
    pub fn append_highlight(&mut self, highlight: Option<HighlightEntity>) {
        self.highlight = highlight
    }

    pub fn exclude_tokens(&mut self) {
        self.content_vector = Vec::default();
    }
}

#[derive(Builder, Clone, Deserialize, Serialize)]
pub struct OcrMetadata {
    pub job_id: String,
    pub text: String,
    pub pages_count: i32,
    pub doc_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

#[derive(Builder, Clone, Deserialize, Serialize)]
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
