use rstest::fixture;

use crate::application::tests::fixture::DEFAULT_INDEX_ID;
use crate::shared::kernel::IndexId;

#[fixture]
pub fn build_index() -> IndexId {
    IndexId(DEFAULT_INDEX_ID.to_string())
}
