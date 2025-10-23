use serde_derive::{Deserialize, Serialize};

use crate::application::structures::{Resource, ResourceBuilder};

#[derive(Serialize, Deserialize)]
pub struct GetUserResourcesForm {
    user_id: String,
    with_public: Option<bool>,
}

impl GetUserResourcesForm {
    pub fn new(user_id: String) -> Self {
        GetUserResourcesForm {
            user_id,
            with_public: Some(true),
        }
    }
}

#[derive(Deserialize)]
pub struct ResourceSchema {
    id: String,
    name: String,
    created_at: chrono::NaiveDateTime,
    is_public: bool,
}

impl From<ResourceSchema> for Resource {
    fn from(schema: ResourceSchema) -> Self {
        ResourceBuilder::default()
            .id(schema.id)
            .name(schema.name)
            .created_at(schema.created_at)
            .is_public(schema.is_public)
            .build()
            .unwrap()
    }
}
