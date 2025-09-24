#![allow(dead_code)]

use gset::Getset;
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

const DEFAULT_LIFETIME: &str = "5m";

#[derive(Deserialize, Serialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CreateDocumentQuery {
    pub force: Option<bool>,
}

#[derive(Deserialize, Serialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PaginateQuery {
    pub lifetime: Option<String>,
}

impl PaginateQuery {
    pub fn lifetime(&self) -> String {
        self.lifetime
            .as_deref()
            .unwrap_or(DEFAULT_LIFETIME)
            .to_string()
    }
}
