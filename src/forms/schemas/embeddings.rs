use crate::forms::schemas::{AsDateField, FieldType};
use crate::forms::schemas::SchemaFieldType;
use crate::forms::schemas::SettingsSchema;

use serde_derive::Serialize;

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
