use crate::forms::schemas::document::ArtifactsSchema;
use crate::forms::schemas::AsDateField;
use crate::forms::schemas::EnabledFlag;
use crate::forms::schemas::FieldType;
use crate::forms::schemas::SchemaFieldType;

use serde_derive::Serialize;

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
