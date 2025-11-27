use derive_builder::Builder;
use doc_search_core::domain::storage::models::IndexId;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Builder, Serialize, Deserialize, ToSchema)]
pub struct IndexSchema {
    #[schema(example = "test-folder")]
    id: String,
}

impl From<IndexSchema> for IndexId {
    fn from(form: IndexSchema) -> Self {
        form.id
    }
}

impl From<IndexId> for IndexSchema {
    fn from(index: IndexId) -> Self {
        IndexSchemaBuilder::default()
            .id(index)
            .build()
            .expect("failed to build IndexSchema from index id")
    }
}
