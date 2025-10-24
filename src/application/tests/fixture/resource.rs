use chrono::Utc;
use rstest::fixture;

use crate::application::structures::{Resource, ResourceBuilder};

pub const DEFAULT_RESOURCE_ID: &str = "sd7ftg2i3bje2";
pub const DEFAULT_RESOURCE_NAME: &str = "test-folder";

#[fixture]
pub fn build_resource() -> Resource {
    ResourceBuilder::default()
        .id(DEFAULT_RESOURCE_ID.to_string())
        .name(DEFAULT_RESOURCE_NAME.to_string())
        .is_public(false)
        .created_at(Utc::now().naive_utc())
        .build()
        .unwrap()
}
