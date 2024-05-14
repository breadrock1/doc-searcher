use crate::forms::documents::document::{Artifacts, Document, GroupValueBuilder};
use crate::forms::TestExample;

use datetime::deserialize_dt;
use datetime::serialize_dt;

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

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
    pub fn builder() -> DocumentPreviewBuilder {
        DocumentPreviewBuilder::default()
    }

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

        DocumentPreview::builder()
            .id("98ac9896be35f47fb8442580cd9839b4".to_string())
            .name("test_document.txt".to_string())
            .created_at(Some(created))
            .quality_recognition(Some(10000))
            .file_size(35345)
            .location("test_folder".to_string())
            .preview_properties(
                vec![Artifacts::builder()
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
        let artifacts = match value.get_ocr_metadata() {
            Some(metadata) => metadata.get_artifacts().cloned(),
            None => None,
        };

        DocumentPreview::builder()
            .id(value.get_doc_md5().to_string())
            .name(value.get_doc_name().to_string())
            .location(value.get_folder_id().to_string())
            .created_at(value.get_doc_created().cloned())
            .quality_recognition(value.get_ocr_quality())
            .file_size(value.get_doc_size())
            .preview_properties(artifacts)
            .build()
            .unwrap()
    }
}
