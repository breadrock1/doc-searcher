use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Builder, ToSchema)]
pub struct PaginatedResult<D> {
    #[schema(value_type = PagintatedResult<Vec<Document>>)]
    founded: D,
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}

impl<D> PaginatedResult<D> {
    pub fn new(founded: D) -> Self {
        PaginatedResult {
            founded: founded,
            scroll_id: None,
        }
    }

    pub fn new_with_id(founded: D, id: String) -> Self {
        PaginatedResult {
            founded: founded,
            scroll_id: Some(id),
        }
    }

    pub fn new_with_opt_id(founded: D, opt_id: Option<String>) -> Self {
        PaginatedResult {
            founded: founded,
            scroll_id: opt_id,
        }
    }

    pub fn get_founded(&self) -> &D {
        &self.founded
    }

    pub fn get_founded_mut(&mut self) -> &mut D {
        &mut self.founded
    }
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct NextScroll {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmZsdnhOSWhk")]
    scroll_id: String,
    #[schema(example = "1m")]
    scroll: String,
}

impl NextScroll {
    pub fn new(id: String, timelife: String) -> Self {
        NextScroll {
            scroll_id: id,
            scroll: timelife,
        }
    }

    pub fn get_scroll_id(&self) -> &str {
        self.scroll_id.as_str()
    }

    pub fn get_timelife(&self) -> &str {
        self.scroll.as_str()
    }
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct AllScrolls {
    scroll_ids: Vec<String>,
}

impl AllScrolls {
    pub fn get_ids(&self) -> &Vec<String> {
        &self.scroll_ids
    }

    pub fn as_slice(&self) -> Vec<&str> {
        self.scroll_ids
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
    }
}
