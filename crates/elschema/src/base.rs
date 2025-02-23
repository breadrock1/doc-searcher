use serde::Serializer;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct EnabledFlag {
    enabled: bool,
}

#[allow(dead_code)]
impl EnabledFlag {
    pub fn new(is_enabled: bool) -> Self {
        EnabledFlag {
            enabled: is_enabled,
        }
    }
}

#[derive(Serialize)]
pub struct SettingsSchema {
    number_of_shards: i32,
    number_of_replicas: i32,
}

impl Default for SettingsSchema {
    fn default() -> Self {
        SettingsSchema {
            number_of_shards: 1,
            number_of_replicas: 1,
        }
    }
}

#[derive(Clone, Default)]
pub enum FieldType {
    Date,
    DenseVector,
    Integer,
    Object,
    Nested,
    #[default]
    Keyword,
    Text,
    Boolean,
}

impl serde::Serialize for FieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let field_type_str = match self {
            FieldType::Date => "date",
            FieldType::Text => "text",
            FieldType::Object => "object",
            FieldType::Nested => "nested",
            FieldType::Boolean => "boolean",
            FieldType::Integer => "integer",
            FieldType::Keyword => "keyword",
            FieldType::DenseVector => "dense_vector",
        };

        serializer.collect_str(field_type_str)
    }
}

#[derive(Serialize)]
pub struct SchemaFieldType {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
}

impl SchemaFieldType {
    pub fn new(field_type: FieldType) -> Self {
        SchemaFieldType { field_type }
    }
}

#[derive(Serialize)]
pub struct AsDateField {
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
