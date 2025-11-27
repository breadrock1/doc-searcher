use derive_builder::Builder;

use crate::domain::searcher::models::document::FoundedDocument;

#[derive(Builder)]
pub struct Pagination {
    pub scroll_id: Option<String>,
    pub founded: Vec<FoundedDocument>,
}

impl Pagination {
    pub fn new(scroll_id: Option<String>, founded: Vec<FoundedDocument>) -> Self {
        Self { scroll_id, founded }
    }
}
