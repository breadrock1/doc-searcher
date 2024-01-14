use serde_derive::Serialize;

#[derive(Serialize)]
pub struct BucketSchema {
    _source: EnabledFlag,
    properties: PropertiesSchema,
}

impl BucketSchema {
    pub fn new() -> Self {
        BucketSchema {
            _source: EnabledFlag::disabled(),
            properties: PropertiesSchema::default(),
        }
    }
}

#[derive(Serialize)]
struct EnabledFlag {
    enabled: bool,
}

impl EnabledFlag {
    pub fn enabled() -> Self {
        EnabledFlag { enabled: true }
    }

    pub fn disabled() -> Self {
        EnabledFlag { enabled: false }
    }
}

#[derive(Serialize)]
struct SchemaFieldType {
    #[serde(alias = "type")]
    _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<String>,
}

enum FieldType {
    Date,
    Integer,
    String,
}

impl SchemaFieldType {
    pub fn new(type_value: FieldType) -> Self {
        let field_type = Self::get_type_str(type_value);
        SchemaFieldType {
            _type: field_type,
            index: None,
        }
    }

    pub fn new_analyzed(type_value: FieldType, analyzed: bool) -> Self {
        let field_type = Self::get_type_str(type_value);
        SchemaFieldType {
            _type: field_type,
            index: {
                let is_analyzed = match analyzed {
                    true => "analyzed",
                    false => "not_analyzed",
                };

                Some(is_analyzed.to_string())
            },
        }
    }

    fn get_type_str(type_value: FieldType) -> String {
        match type_value {
            FieldType::Integer => "integer",
            FieldType::String => "string",
            FieldType::Date => "date",
        }
        .to_string()
    }
}

#[derive(Serialize)]
struct PropertiesSchema {
    _timestamp: EnabledFlag,
    bucket_uuid: SchemaFieldType,
    bucket_path: SchemaFieldType,
    document_name: SchemaFieldType,
    document_path: SchemaFieldType,
    document_size: SchemaFieldType,
    document_type: SchemaFieldType,
    document_extension: SchemaFieldType,
    document_permissions: SchemaFieldType,
    document_md5_hash: SchemaFieldType,
    document_ssdeep_hash: SchemaFieldType,
    entity_data: SchemaFieldType,
    entity_keywords: Vec<String>,
    document_created: SchemaFieldType,
    document_modified: SchemaFieldType,
}

impl Default for PropertiesSchema {
    fn default() -> Self {
        PropertiesSchema {
            _timestamp: EnabledFlag::enabled(),
            bucket_uuid: SchemaFieldType::new(FieldType::String),
            bucket_path: SchemaFieldType::new(FieldType::String),
            document_path: SchemaFieldType::new_analyzed(FieldType::String, false),
            document_name: SchemaFieldType::new(FieldType::String),
            document_size: SchemaFieldType::new(FieldType::Integer),
            document_type: SchemaFieldType::new(FieldType::String),
            document_extension: SchemaFieldType::new(FieldType::String),
            document_permissions: SchemaFieldType::new(FieldType::Integer),
            document_md5_hash: SchemaFieldType::new(FieldType::String),
            document_ssdeep_hash: SchemaFieldType::new(FieldType::String),
            document_created: SchemaFieldType::new(FieldType::Date),
            document_modified: SchemaFieldType::new(FieldType::Date),
            entity_data: SchemaFieldType::new(FieldType::String),
            entity_keywords: Vec::default(),
        }
    }
}
