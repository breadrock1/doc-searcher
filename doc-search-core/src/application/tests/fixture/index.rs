use rstest::fixture;

use crate::domain::storage::models::IndexId;

pub const DEFAULT_INDEX_ID: &str = "test-folder";

#[fixture]
pub fn build_index() -> IndexId {
    DEFAULT_INDEX_ID.to_string()
}
