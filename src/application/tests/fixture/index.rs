use rstest::fixture;

use crate::application::structures::{Index, IndexBuilder};

pub const DEFAULT_INDEX_ID: &str = "test-folder";
pub const DEFAULT_INDEX_PATH: &str = "./";

#[fixture]
pub fn build_index() -> Index {
    IndexBuilder::default()
        .id(DEFAULT_INDEX_ID.to_string())
        .name(DEFAULT_INDEX_ID.to_string())
        .path(DEFAULT_INDEX_PATH.to_string())
        .build()
        .unwrap()
}
