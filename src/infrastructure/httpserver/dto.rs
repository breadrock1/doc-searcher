use getset::CopyGetters;
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize)]
pub struct PaginateQuery {
    lifetime: Option<String>,
}

impl PaginateQuery {
    pub fn lifetime(&self) -> String {
        self.lifetime.clone().unwrap_or("5m".to_string())
    }
}

#[derive(Deserialize, Serialize, CopyGetters, IntoParams, ToSchema)]
pub struct CreateDocumentQuery {
    #[getset(get_copy = "pub")]
    #[schema(example = false)]
    force: Option<bool>,
}
