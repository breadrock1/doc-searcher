use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct CreateClusterForm {
    #[schema(example = "test_slave")]
    cluster_id: String,
    #[schema(example = "slave")]
    role: String,
}

impl CreateClusterForm {
    pub fn builder() -> CreateClusterFormBuilder {
        CreateClusterFormBuilder::default()
    }
}
