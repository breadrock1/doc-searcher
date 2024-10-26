use elschema::base::{AsDateField, FieldType, SchemaFieldType, SettingsSchema};
use elschema::embeddings::EmbeddingsSchema;
use elschema::ElasticSchema;
use serde::Serialize;

#[derive(Serialize)]
pub struct InfoFolderSchema {
    settings: SettingsSchema,
    mappings: InfoFolderMappings,
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

impl ElasticSchema for InfoFolderSchema {
    fn build() -> InfoFolderSchema {
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
pub struct DocumentSchema {
    settings: SettingsSchema,
    mappings: DocumentMappings,
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
    embeddings: EmbeddingsSchema,
}

impl ElasticSchema for DocumentSchema {
    fn build() -> Self {
        DocumentSchema {
            settings: SettingsSchema::default(),
            mappings: DocumentMappings {
                properties: DocumentProperties {
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
                    embeddings: EmbeddingsSchema::default(),
                },
            },
        }
    }
}

#[derive(Serialize)]
pub struct DocumentVectorSchema {
    settings: SettingsSchema,
    mappings: DocumentVectorMappings,
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

impl ElasticSchema for DocumentVectorSchema {
    fn build() -> Self {
        let text_vec_properties = TextVectorProperties {
            vector: SchemaFieldType::new(FieldType::DenseVector),
            text_chunk: SchemaFieldType::new(FieldType::Text),
        };

        DocumentVectorSchema {
            settings: SettingsSchema::default(),
            mappings: DocumentVectorMappings {
                properties: DocumentVectorProperties {
                    folder_id: SchemaFieldType::new(FieldType::Keyword),
                    document_id: SchemaFieldType::new(FieldType::Keyword),
                    document_name: SchemaFieldType::new(FieldType::Keyword),
                    document_modified: AsDateField::default(),
                    embeddings: TextVectorSchema {
                        field_type: FieldType::Nested,
                        properties: text_vec_properties,
                    },
                },
            },
        }
    }
}
