use crate::forms::TestExample;

use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct PaginateNextForm {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    scroll_id: String,
    #[schema(example = "1m")]
    lifetime: String,
}

impl PaginateNextForm {
    pub fn get_scroll_id(&self) -> &str {
        self.scroll_id.as_str()
    }
    pub fn get_lifetime(&self) -> &str {
        self.lifetime.as_str()
    }
}

impl TestExample<PaginateNextForm> for PaginateNextForm {
    fn test_example(_value: Option<&str>) -> PaginateNextForm {
        PaginateNextForm {
            scroll_id: "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk".to_string(),
            lifetime: "1m".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct DeletePaginationsForm {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    sessions: Vec<String>,
}

impl DeletePaginationsForm {
    pub fn get_sessions(&self) -> Vec<&str> {
        self.sessions.iter().map(String::as_str).collect()
    }
}

impl TestExample<DeletePaginationsForm> for DeletePaginationsForm {
    fn test_example(_value: Option<&str>) -> DeletePaginationsForm {
        let id = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk";
        DeletePaginationsForm {
            sessions: vec![id.to_string()],
        }
    }
}
