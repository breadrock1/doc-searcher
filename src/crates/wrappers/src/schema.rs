use serde_derive::Serialize;

#[derive(Default, Serialize)]
pub struct BucketSchema {
    _source: EnabledFlag,
    properties: PropertiesSchema,
}

#[derive(Serialize)]
struct PropertiesSchema {
    _timestamp: EnabledFlag,

    bucket_uuid: SchemaFieldType,
    bucket_path: SchemaFieldType,

    content: SchemaFieldType,
    content_md5: SchemaFieldType,
    content_uuid: SchemaFieldType,
    content_vector: SchemaFieldType,

    document_md5: SchemaFieldType,
    document_ssdeep: SchemaFieldType,
    document_name: SchemaFieldType,
    document_path: SchemaFieldType,
    document_size: SchemaFieldType,
    document_type: SchemaFieldType,
    document_extension: SchemaFieldType,
    document_permissions: SchemaFieldType,
    document_created: SchemaFieldType,
    document_modified: SchemaFieldType,
}

impl Default for PropertiesSchema {
    fn default() -> Self {
        PropertiesSchema {
            _timestamp: EnabledFlag::enabled(),

            bucket_uuid: SchemaFieldType::new(FieldType::String),
            bucket_path: SchemaFieldType::new(FieldType::String),

            content: SchemaFieldType::new(FieldType::String),
            content_md5: SchemaFieldType::new(FieldType::String),
            content_uuid: SchemaFieldType::new(FieldType::String),
            content_vector: SchemaFieldType::new_dense(FieldType::Dense, 1024),

            document_md5: SchemaFieldType::new(FieldType::String),
            document_ssdeep: SchemaFieldType::new(FieldType::String),
            document_path: SchemaFieldType::new_analyzed(FieldType::String, false),
            document_name: SchemaFieldType::new(FieldType::String),
            document_size: SchemaFieldType::new(FieldType::Integer),
            document_type: SchemaFieldType::new(FieldType::String),
            document_extension: SchemaFieldType::new(FieldType::String),
            document_permissions: SchemaFieldType::new(FieldType::Integer),
            document_created: SchemaFieldType::new(FieldType::Date),
            document_modified: SchemaFieldType::new(FieldType::Date),
        }
    }
}

enum FieldType {
    Date,
    Dense,
    Integer,
    String,
}

#[derive(Serialize)]
struct SchemaFieldType {
    #[serde(rename(serialize = "type"))]
    field_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dims: Option<u32>,
}

impl SchemaFieldType {
    pub fn new(type_value: FieldType) -> Self {
        let field_type = Self::get_type_str(type_value);
        SchemaFieldType {
            field_type,
            index: None,
            dims: None,
        }
    }

    pub fn new_analyzed(type_value: FieldType, analyzed: bool) -> Self {
        let field_type = Self::get_type_str(type_value);
        SchemaFieldType {
            field_type,
            dims: None,
            index: {
                let is_analyzed = match analyzed {
                    true => "analyzed",
                    false => "not_analyzed",
                };

                Some(is_analyzed.to_string())
            },
        }
    }

    pub fn new_dense(type_value: FieldType, dense_size: u32) -> Self {
        let field_type = Self::get_type_str(type_value);
        SchemaFieldType {
            field_type,
            index: None,
            dims: Some(dense_size),
        }
    }

    fn get_type_str(type_value: FieldType) -> String {
        match type_value {
            FieldType::Integer => "integer",
            FieldType::String => "string",
            FieldType::Dense => "dense_vector",
            FieldType::Date => "date",
        }
        .to_string()
    }
}

#[derive(Default, Serialize)]
struct EnabledFlag {
    enabled: bool,
}

impl EnabledFlag {
    pub fn enabled() -> Self {
        EnabledFlag { enabled: true }
    }

    pub fn _disabled() -> Self {
        EnabledFlag { enabled: false }
    }
}
