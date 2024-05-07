use serde_derive::Serialize;

enum FieldType {
    Date,
    Dense,
    Integer,
    String,
    Object,
}

#[derive(Serialize)]
pub struct PreviewDocumentSchema {
    _source: EnabledFlag,
    properties: PropertiesSchema
}

impl Default for PreviewDocumentSchema {
    fn default() -> Self {
        PreviewDocumentSchema {
            _source: EnabledFlag::enabled(),
            properties: PropertiesSchema::default(),
        }
    }
}

#[derive(Serialize)]
struct PreviewPropertiesSchema {
    id: SchemaFieldType,
    name: SchemaFieldType,
    location: SchemaFieldType,
    file_size: SchemaFieldType,
    created_at: SchemaFieldType,
    preview_properties: SchemaFieldType,
    quality_recognition: SchemaFieldType,
}

impl Default for PreviewPropertiesSchema {
    fn default() -> Self {
        PreviewPropertiesSchema {
            id: SchemaFieldType::new(FieldType::String),
            name: SchemaFieldType::new(FieldType::String),
            location: SchemaFieldType::new(FieldType::String),
            created_at: SchemaFieldType::new(FieldType::Date),
            file_size: SchemaFieldType::new(FieldType::Integer),
            quality_recognition: SchemaFieldType::new(FieldType::Integer),
            preview_properties: SchemaFieldType::new_dynamic(FieldType::Object),
        }
    }
}



#[derive(Serialize)]
pub struct DocumentSchema {
    _source: EnabledFlag,
    properties: PropertiesSchema,
}

impl Default for DocumentSchema {
    fn default() -> Self {
        DocumentSchema {
            _source: EnabledFlag::enabled(),
            properties: PropertiesSchema::default(),
        }
    }
}

#[derive(Serialize)]
struct PropertiesSchema {
    folder_id: SchemaFieldType,
    folder_path: SchemaFieldType,
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
    quality_recognition: SchemaFieldType,
    ocr_metadata: SchemaFieldType,
}

impl Default for PropertiesSchema {
    fn default() -> Self {
        PropertiesSchema {
            folder_id: SchemaFieldType::new(FieldType::String),
            folder_path: SchemaFieldType::new(FieldType::String),
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
            quality_recognition: SchemaFieldType::new(FieldType::Integer),
            ocr_metadata: SchemaFieldType::new_dynamic(FieldType::Object),
        }
    }
}

#[derive(Serialize)]
struct OcrMetadataSchema {
    job_id: SchemaFieldType,
    text: SchemaFieldType,
    pages_count: SchemaFieldType,
    doc_type: SchemaFieldType,
    artifacts: SchemaFieldType,
}

#[derive(Serialize)]
struct SchemaFieldType {
    #[serde(rename(serialize = "type"))]
    field_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dims: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic: Option<bool>,
}

impl SchemaFieldType {
    pub fn new(type_value: FieldType) -> Self {
        let field_type = Self::get_type_str(type_value);
        SchemaFieldType {
            field_type,
            index: None,
            dims: None,
            dynamic: None,
        }
    }

    pub fn new_analyzed(type_value: FieldType, analyzed: bool) -> Self {
        let field_type = Self::get_type_str(type_value);
        SchemaFieldType {
            field_type,
            dims: None,
            dynamic: None,
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
            dynamic: None,
            dims: Some(dense_size),
        }
    }

    pub fn new_dynamic(type_value: FieldType) -> Self {
        let field_type = Self::get_type_str(type_value);
        SchemaFieldType {
            field_type,
            index: None,
            dims: None,
            dynamic: Some(true),
        }
    }

    fn get_type_str(type_value: FieldType) -> String {
        match type_value {
            FieldType::Integer => "integer",
            FieldType::String => "string",
            FieldType::Dense => "dense_vector",
            FieldType::Date => "date",
            FieldType::Object => "object",
        }
        .to_string()
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

    pub fn _disabled() -> Self {
        EnabledFlag { enabled: false }
    }
}
