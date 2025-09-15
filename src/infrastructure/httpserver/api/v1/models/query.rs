#![allow(dead_code)]

use gset::Getset;
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

const DEFAULT_LIFETIME: &str = "5m";

#[derive(Deserialize, Serialize, Getset, IntoParams, ToSchema)]
pub struct CreateDocumentQuery {
    #[getset(get_copy, vis = "pub")]
    #[schema(example = false)]
    force: Option<bool>,
}

#[derive(Deserialize)]
pub struct PaginateQuery {
    lifetime: Option<String>,
}

impl PaginateQuery {
    pub fn lifetime(&self) -> String {
        self.lifetime
            .as_deref()
            .unwrap_or(DEFAULT_LIFETIME)
            .to_string()
    }
}
