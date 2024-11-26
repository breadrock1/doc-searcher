use crate::base::{FieldType, SchemaFieldType};

use serde_derive::Serialize;

#[derive(Serialize)]
pub struct EmbeddingsSchema {
    #[serde(rename(serialize = "type"))]
    field_type: FieldType,
    properties: EmbeddingProperties,
}

impl Default for EmbeddingsSchema {
    fn default() -> Self {
        EmbeddingsSchema {
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
