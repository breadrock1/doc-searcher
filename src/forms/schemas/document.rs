use crate::forms::schemas::AsDateField;
use crate::forms::schemas::EnabledFlag;
use crate::forms::schemas::FieldIndex;
use crate::forms::schemas::FieldType;
use crate::forms::schemas::SchemaFieldType;

use serde_derive::Serialize;

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
pub(crate) struct ArtifactsSchema {
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
                name: SchemaFieldType::new(FieldType::String),
                json_name: SchemaFieldType::new(FieldType::String),
                group_type: SchemaFieldType::new(FieldType::Keyword),
                value: group_values_fields,
            },
        }
    }
}
