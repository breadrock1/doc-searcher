use derive_builder::Builder;
use serde::Serializer;
use serde_derive::Serialize;

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
pub struct DocumentSchema {
    _source: EnabledFlag,
    properties: DocumentProperties,
}

#[derive(Serialize)]
struct DocumentProperties {
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
    quality_recognition: SchemaFieldType,
    document_created: AsDateField,
    document_modified: AsDateField,
    ocr_metadata: OcrMetadataSchema,
}

impl Default for DocumentSchema {
    fn default() -> Self {
        let doc_path_field = SchemaFieldType::builder()
            .field_type(FieldType::String)
            .index(Some(FieldIndex::NotAnalyzed))
            .dynamic(None)
            .dims(None)
            .build()
            .unwrap();

        let content_vector_field = SchemaFieldType::builder()
            .field_type(FieldType::DenseVector)
            .index(None)
            .dynamic(None)
            .dims(None)
            .build()
            .unwrap();

        DocumentSchema {
            _source: EnabledFlag::new(true),
            properties: DocumentProperties {
                folder_id: SchemaFieldType::new(FieldType::String),
                folder_path: SchemaFieldType::new(FieldType::String),
                content: SchemaFieldType::new(FieldType::Text),
                content_md5: SchemaFieldType::new(FieldType::String),
                content_uuid: SchemaFieldType::new(FieldType::String),
                document_md5: SchemaFieldType::new(FieldType::String),
                document_ssdeep: SchemaFieldType::new(FieldType::String),
                document_name: SchemaFieldType::new(FieldType::String),
                document_size: SchemaFieldType::new(FieldType::Integer),
                document_type: SchemaFieldType::new(FieldType::Keyword),
                document_extension: SchemaFieldType::new(FieldType::Keyword),
                document_permissions: SchemaFieldType::new(FieldType::Integer),
                quality_recognition: SchemaFieldType::new(FieldType::Integer),

                document_path: doc_path_field,
                content_vector: content_vector_field,

                document_created: AsDateField::default(),
                document_modified: AsDateField::default(),
                ocr_metadata: OcrMetadataSchema::default(),
            },
        }
    }
}

#[derive(Serialize)]
struct OcrMetadataSchema {
    field_type: FieldType,
    properties: OcrMetadataProperties,
}

#[derive(Serialize)]
struct OcrMetadataProperties {
    job_id: SchemaFieldType,
    text: SchemaFieldType,
    pages_count: SchemaFieldType,
    doc_type: SchemaFieldType,
    artifacts: ArtifactsSchema,
}

impl Default for OcrMetadataSchema {
    fn default() -> Self {
        OcrMetadataSchema {
            field_type: FieldType::Object,
            properties: OcrMetadataProperties {
                job_id: SchemaFieldType::new(FieldType::String),
                text: SchemaFieldType::new(FieldType::String),
                doc_type: SchemaFieldType::new(FieldType::String),
                pages_count: SchemaFieldType::new(FieldType::Integer),
                artifacts: ArtifactsSchema::default(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct DocumentPreviewSchema {
    _source: EnabledFlag,
    properties: DocumentPreviewProperties,
}

#[derive(Serialize)]
struct DocumentPreviewProperties {
    id: SchemaFieldType,
    name: SchemaFieldType,
    quality_recognition: SchemaFieldType,
    file_size: SchemaFieldType,
    location: SchemaFieldType,
    created_at: AsDateField,
    artifacts: ArtifactsSchema,
}

impl Default for DocumentPreviewSchema {
    fn default() -> Self {
        DocumentPreviewSchema {
            _source: EnabledFlag::new(true),
            properties: DocumentPreviewProperties {
                id: SchemaFieldType::new(FieldType::String),
                name: SchemaFieldType::new(FieldType::String),
                location: SchemaFieldType::new(FieldType::String),
                file_size: SchemaFieldType::new(FieldType::Integer),
                quality_recognition: SchemaFieldType::new(FieldType::Integer),
                created_at: AsDateField::default(),
                artifacts: ArtifactsSchema::default(),
            },
        }
    }
}

#[derive(Serialize)]
struct ArtifactsSchema {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    properties: ArtifactsProperties,
}

#[derive(Serialize)]
struct ArtifactsProperties {
    group_name: SchemaFieldType,
    group_json_name: SchemaFieldType,
    group_values: GroupValues,
}

impl Default for ArtifactsSchema {
    fn default() -> Self {
        ArtifactsSchema {
            field_type: FieldType::Nested,
            properties: ArtifactsProperties {
                group_name: SchemaFieldType::new(FieldType::String),
                group_json_name: SchemaFieldType::new(FieldType::String),
                group_values: GroupValues::default(),
            },
        }
    }
}

#[derive(Serialize)]
struct GroupValues {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    properties: GroupValuesPeroperties,
}

#[derive(Serialize)]
struct GroupValuesPeroperties {
    name: SchemaFieldType,
    json_name: SchemaFieldType,
    #[serde(rename(serialize = "type"))]
    group_type: SchemaFieldType,
    value: GroupValueFields,
}

#[derive(Serialize)]
struct GroupValueFields {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    fields: AsDateField,
}

impl Default for GroupValues {
    fn default() -> Self {
        let group_values_fields = GroupValueFields {
            field_type: FieldType::Text,
            fields: AsDateField::default(),
        };

        GroupValues {
            field_type: FieldType::Nested,
            properties: GroupValuesPeroperties {
                name: SchemaFieldType::new(FieldType::String),
                json_name: SchemaFieldType::new(FieldType::String),
                group_type: SchemaFieldType::new(FieldType::Keyword),
                value: group_values_fields,
            },
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

#[derive(Serialize)]
pub struct DocumentVectorSchema {
    mappings: DocumentVectorMappings,
}

#[derive(Serialize)]
struct DocumentVectorMappings {
    properties: DocumentVectorProperties,
}

#[derive(Serialize)]
struct DocumentVectorProperties {
    text_chunk: SchemaFieldType,
    text_vector: TextVectorSchema,
}

#[derive(Serialize)]
struct TextVectorSchema {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    properties: TextVectorProperties,
}

#[derive(Serialize)]
struct TextVectorProperties {
    vector: SchemaFieldType,
}

impl Default for DocumentVectorSchema {
    fn default() -> Self {
        let text_vec_properties = TextVectorProperties {
            vector: SchemaFieldType::new(FieldType::DenseVector),
        };

        let doc_vec_properties = DocumentVectorProperties {
            text_chunk: SchemaFieldType::new(FieldType::Text),
            text_vector: TextVectorSchema {
                field_type: FieldType::Nested,
                properties: text_vec_properties,
            },
        };

        DocumentVectorSchema {
            mappings: DocumentVectorMappings {
                properties: doc_vec_properties,
            },
        }
    }
}

pub trait ElasticSchema {}
impl ElasticSchema for DocumentSchema {}
impl ElasticSchema for DocumentVectorSchema {}
impl ElasticSchema for DocumentPreviewSchema {}
