use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

use doc_search_core::shared::kernel::IndexId;

#[derive(Clone, Builder, Serialize, Deserialize, ToSchema)]
pub struct IndexSchema {
    #[schema(example = "test-folder")]
    pub id: String,
}

impl From<IndexSchema> for IndexId {
    fn from(form: IndexSchema) -> Self {
        IndexId(form.id)
    }
}

impl From<IndexId> for IndexSchema {
    fn from(index: IndexId) -> Self {
        IndexSchemaBuilder::default()
            .id(index.0)
            .build()
            .expect("failed to build IndexSchema from index id")
    }
}
