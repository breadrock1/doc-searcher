use crate::forms::TestExample;

use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct CreateClusterForm {
    #[schema(example = "test_slave")]
    cluster_id: String,
}

impl CreateClusterForm {
    #[allow(dead_code)]
    pub fn get_id(&self) -> &str {
        self.cluster_id.as_str()
    }
}

impl Display for CreateClusterForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_data = &self.cluster_id;
        write!(f, "{}", self_data.clone())
    }
}

impl TestExample<CreateClusterForm> for CreateClusterForm {
    fn test_example(_value: Option<&str>) -> CreateClusterForm {
        CreateClusterForm {
            cluster_id: "test-slave-cluster".to_string(),
        }
    }
}
