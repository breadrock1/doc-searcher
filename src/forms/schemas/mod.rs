pub(crate) mod document;
pub(crate) mod preview;
pub(crate) mod vector;

use crate::forms::schemas::document::DocumentSchema;
use crate::forms::schemas::preview::DocumentPreviewSchema;
use crate::forms::schemas::vector::DocumentVectorSchema;

use derive_builder::Builder;
use serde::Serializer;
use serde_derive::Serialize;

pub trait ElasticSchema {}
impl ElasticSchema for DocumentSchema {}
impl ElasticSchema for DocumentVectorSchema {}
impl ElasticSchema for DocumentPreviewSchema {}

#[derive(Clone, Serialize)]
enum FieldIndex {
    #[serde(rename(serialize = "analyzed"))]
    Analyzed,
    #[serde(rename(serialize = "not_analyzed"))]
    NotAnalyzed,
}

impl Default for FieldIndex {
    fn default() -> Self {
        FieldIndex::Analyzed
    }
}

#[derive(Serialize)]
struct EnabledFlag {
    enabled: bool,
}

impl EnabledFlag {
    pub fn new(is_enabled: bool) -> Self {
        EnabledFlag {
            enabled: is_enabled,
        }
    }
}

#[derive(Clone, Default)]
enum FieldType {
    Date,
    DenseVector,
    Integer,
    #[default]
    String,
    Object,
    Nested,
    Keyword,
    Text,
}

impl serde::Serialize for FieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let field_type_str = match self {
            FieldType::Date => "date",
            FieldType::Text => "text",
            FieldType::String => "string",
            FieldType::Object => "object",
            FieldType::Nested => "nested",
            FieldType::Integer => "integer",
            FieldType::Keyword => "keyword",
            FieldType::DenseVector => "dense_vector",
        };

        serializer.collect_str(field_type_str)
    }
}

#[derive(Builder, Default, Serialize)]
struct SchemaFieldType {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<FieldIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dims: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic: Option<bool>,
}

impl SchemaFieldType {
    pub fn builder() -> SchemaFieldTypeBuilder {
        SchemaFieldTypeBuilder::default()
    }

    pub fn new(field_type: FieldType) -> Self {
        SchemaFieldType {
            field_type: field_type,
            ..Default::default()
        }
    }
}

#[derive(Serialize)]
struct AsDateField {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    ignore_malformed: bool,
}

impl Default for AsDateField {
    fn default() -> Self {
        AsDateField {
            field_type: FieldType::Date,
            ignore_malformed: true,
        }
    }
}
