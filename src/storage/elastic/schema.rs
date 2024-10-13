use serde::Serializer;
use serde_derive::Serialize;

#[allow(dead_code)]
pub trait ElasticSchema {}
impl ElasticSchema for DocumentSchema {}
impl ElasticSchema for DocumentVectorSchema {}

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

#[derive(Serialize)]
pub struct InfoFolderSchema {
    settings: SettingsSchema,
    mappings: InfoFolderMappings,
}

impl Default for InfoFolderSchema {
    fn default() -> Self {
        InfoFolderSchema {
            settings: SettingsSchema::default(),
            mappings: InfoFolderMappings {
                properties: InfoFolderProperties {
                    index: SchemaFieldType::new(FieldType::Text),
                    name: SchemaFieldType::new(FieldType::Text),
                    user_id: SchemaFieldType::new(FieldType::Text),
                    document_type: SchemaFieldType::new(FieldType::Text),
                    is_system: SchemaFieldType::new(FieldType::Boolean),
                },
            },
        }
    }
}

#[derive(Serialize)]
struct InfoFolderMappings {
    properties: InfoFolderProperties,
}

#[derive(Serialize)]
struct InfoFolderProperties {
    index: SchemaFieldType,
    name: SchemaFieldType,
    user_id: SchemaFieldType,
    document_type: SchemaFieldType,
    is_system: SchemaFieldType,
}

#[derive(Serialize)]
pub struct DocumentSchema {
    settings: SettingsSchema,
    mappings: DocumentMappings,
}
impl Default for DocumentSchema {
    fn default() -> Self {
        let doc_properties = DocumentProperties {
            content: SchemaFieldType::new(FieldType::Text),
            folder_id: SchemaFieldType::new(FieldType::Keyword),
            folder_path: SchemaFieldType::new(FieldType::Keyword),
            document_id: SchemaFieldType::new(FieldType::Keyword),
            document_ssdeep: SchemaFieldType::new(FieldType::Keyword),
            document_name: SchemaFieldType::new(FieldType::Keyword),
            document_size: SchemaFieldType::new(FieldType::Integer),
            document_type: SchemaFieldType::new(FieldType::Keyword),
            document_extension: SchemaFieldType::new(FieldType::Keyword),
            document_permissions: SchemaFieldType::new(FieldType::Integer),
            quality_recognition: SchemaFieldType::new(FieldType::Integer),
            document_path: SchemaFieldType::new(FieldType::Keyword),
            document_created: AsDateField::default(),
            document_modified: AsDateField::default(),
            ocr_metadata: OcrMetadataSchema::default(),
            embeddings: EmbeddingSchema::default(),
        };

        DocumentSchema {
            settings: SettingsSchema::default(),
            mappings: DocumentMappings {
                properties: doc_properties,
            },
        }
    }
}

#[derive(Serialize)]
struct DocumentMappings {
    properties: DocumentProperties,
}

#[derive(Serialize)]
struct DocumentProperties {
    folder_id: SchemaFieldType,
    folder_path: SchemaFieldType,
    content: SchemaFieldType,
    document_id: SchemaFieldType,
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
    embeddings: EmbeddingSchema,
}

#[derive(Serialize)]
struct EmbeddingSchema {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    properties: EmbeddingProperties,
}

impl Default for EmbeddingSchema {
    fn default() -> Self {
        EmbeddingSchema {
            field_type: FieldType::Nested,
            properties: EmbeddingProperties {
                text_chunk: SchemaFieldType::new(FieldType::Text),
                vector: VectorSchema::default(),
            },
        }
    }
}

#[derive(Serialize)]
struct EmbeddingProperties {
    text_chunk: SchemaFieldType,
    vector: VectorSchema,
}

#[derive(Serialize)]
struct VectorSchema {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    similarity: String,
    index: bool,
    dims: u32,
}

impl Default for VectorSchema {
    fn default() -> Self {
        VectorSchema {
            field_type: FieldType::DenseVector,
            similarity: "cosine".to_string(),
            index: true,
            dims: 1024,
        }
    }
}

#[derive(Serialize)]
struct OcrMetadataSchema {
    #[serde(rename(serialize = "type"))]
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
                artifacts: ArtifactsSchema::default(),
                text: SchemaFieldType::new(FieldType::Text),
                job_id: SchemaFieldType::new(FieldType::Keyword),
                doc_type: SchemaFieldType::new(FieldType::Keyword),
                pages_count: SchemaFieldType::new(FieldType::Integer),
            },
        }
    }
}

#[derive(Serialize)]
pub struct ArtifactsSchema {
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
                group_name: SchemaFieldType::new(FieldType::Keyword),
                group_json_name: SchemaFieldType::new(FieldType::Keyword),
                group_values: GroupValues::default(),
            },
        }
    }
}

#[derive(Serialize)]
struct GroupValues {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    properties: GroupValuesProperties,
}

#[derive(Serialize)]
struct GroupValuesProperties {
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
            properties: GroupValuesProperties {
                name: SchemaFieldType::new(FieldType::Keyword),
                json_name: SchemaFieldType::new(FieldType::Keyword),
                group_type: SchemaFieldType::new(FieldType::Keyword),
                value: group_values_fields,
            },
        }
    }
}

#[derive(Serialize)]
pub struct DocumentVectorSchema {
    settings: SettingsSchema,
    mappings: DocumentVectorMappings,
}

impl Default for DocumentVectorSchema {
    fn default() -> Self {
        let text_vec_properties = TextVectorProperties {
            vector: SchemaFieldType::new(FieldType::DenseVector),
            text_chunk: SchemaFieldType::new(FieldType::Text),
        };

        let doc_vec_properties = DocumentVectorProperties {
            folder_id: SchemaFieldType::new(FieldType::Keyword),
            document_id: SchemaFieldType::new(FieldType::Keyword),
            document_name: SchemaFieldType::new(FieldType::Keyword),
            document_modified: AsDateField::default(),
            embeddings: TextVectorSchema {
                field_type: FieldType::Nested,
                properties: text_vec_properties,
            },
        };

        DocumentVectorSchema {
            settings: SettingsSchema::default(),
            mappings: DocumentVectorMappings {
                properties: doc_vec_properties,
            },
        }
    }
}

#[derive(Serialize)]
struct DocumentVectorMappings {
    properties: DocumentVectorProperties,
}

#[derive(Serialize)]
struct DocumentVectorProperties {
    folder_id: SchemaFieldType,
    document_id: SchemaFieldType,
    document_name: SchemaFieldType,
    document_modified: AsDateField,
    embeddings: TextVectorSchema,
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
    text_chunk: SchemaFieldType,
}
