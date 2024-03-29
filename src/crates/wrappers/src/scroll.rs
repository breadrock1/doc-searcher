use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Builder)]
pub struct PagintatedResult<D> {
    founded: D,
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_id: Option<String>,
}

impl<D> PagintatedResult<D> {
    pub fn new(founded: D) -> Self {
        PagintatedResult {
            founded: founded,
            scroll_id: None,
        }
    }

    pub fn new_with_id(founded: D, id: String) -> Self {
        PagintatedResult {
            founded: founded,
            scroll_id: Some(id),
        }
    }

    pub fn new_with_opt_id(founded: D, opt_id: Option<String>) -> Self {
        PagintatedResult {
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

#[derive(Deserialize)]
pub struct NextScroll {
    scroll_id: String,
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

#[derive(Deserialize)]
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
