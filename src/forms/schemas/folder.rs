use crate::forms::schemas::{FieldType, SchemaFieldType, SettingsSchema};

use serde_derive::Serialize;

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
                }
            }
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
