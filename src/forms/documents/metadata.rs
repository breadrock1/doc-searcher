use derive_builder::Builder;
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Clone, Deserialize, Serialize, ToSchema)]
pub struct OcrMetadata {
    #[schema(example = "c643c506-f5c3-4262-991d-bbe847035499")]
    job_id: String,
    #[schema(example = 1)]
    pages_count: i32,
    #[schema(example = "Коносамент")]
    doc_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifacts: Option<Vec<Artifacts>>,
}

impl OcrMetadata {
    pub fn builder() -> OcrMetadataBuilder {
        OcrMetadataBuilder::default()
    }
    pub fn get_job_id(&self) -> &str {
        self.job_id.as_str()
    }
    pub fn get_pages_count(&self) -> i32 {
        self.pages_count
    }
    pub fn get_doc_type(&self) -> &str {
        self.doc_type.as_str()
    }
    pub fn get_artifacts(&self) -> Option<&Vec<Artifacts>> {
        self.artifacts.as_ref()
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

impl Artifacts {
    pub fn builder() -> ArtifactsBuilder {
        ArtifactsBuilder::default()
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
    #[serde(deserialize_with = "deser_group_value")]
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
    String::deserialize(deserializer).and_then(|value| Ok(Some(value.replace("-", "   "))))
}

#[derive(Builder, Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct HighlightEntity {
    pub content: Vec<String>,
}
