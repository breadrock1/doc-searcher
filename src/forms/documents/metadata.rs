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
    pub fn set_artifacts(&mut self, artifacts: Option<Vec<Artifacts>>) {
        self.artifacts = artifacts
    }
    pub fn set_doc_type(&mut self, doc_type: &str) {
        self.doc_type = doc_type.to_string();
    }
}

#[allow(dead_code)]
#[derive(Clone, Deserialize, ToSchema)]
pub struct DocsArtifacts {
    name: String,
    json_name: String,
    sample_file_name: String,
    artifacts: Vec<Artifacts>,
}

impl DocsArtifacts {
    pub fn get_artifacts(&self) -> &Vec<Artifacts> {
        &self.artifacts
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
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
