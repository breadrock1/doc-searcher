use rstest::fixture;

use crate::application::tests::fixture::DEFAULT_INDEX_ID;
use crate::domain::storage::models::IndexId;

#[fixture]
pub fn build_index() -> IndexId {
    DEFAULT_INDEX_ID.to_string()
}
